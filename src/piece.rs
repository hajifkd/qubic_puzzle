use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    size: usize,
    pieces: Vec<u8>,
}

#[derive(Debug)]
pub enum Direction {
    X,
    Y,
    Z,
}

const DIMENSION: usize = 3;

impl Piece {
    pub fn new(size: usize, pieces: &[u8]) -> Option<Piece> {
        if size.pow(DIMENSION as _) > pieces.len() {
            let mut pieces = pieces.to_vec();
            pieces.extend((pieces.len()..size.pow(DIMENSION as _)).map(|_| 0));
            Some(Piece {
                size,
                pieces: pieces,
            })
        } else {
            None
        }
    }

    pub fn append(&self, other: &Self) -> Option<Self> {
        let pieces = self
            .indices()
            .map(|(x, y, z)| {
                let s = self.pieces[self.index(x, y, z)];
                let o = other.pieces[other.index(x, y, z)];
                if s == 1 && o == 1 {
                    None
                } else {
                    Some(s + o)
                }
            })
            .collect::<Option<_>>()?;

        Some(Self {
            size: self.size,
            pieces,
        })
    }

    fn indices(&self) -> impl Iterator<Item = (usize, usize, usize)> {
        (0..(self.size.pow(DIMENSION as _))).map(|mut i| {
            let x = i % 3;
            i = i / 3;
            let y = i % 3;
            let z = i / 3;
            (x, y, z)
        })
    }

    pub fn z_rotation(&self) -> Self {
        let pieces = self
            .indices()
            .map(|(x, y, z)| {
                let ox = y;
                let oy = self.size - 1 - x;
                self.pieces[self.index(ox, oy, z)]
            })
            .collect();

        return Piece {
            size: self.size,
            pieces,
        };
    }

    pub fn y_rotation(&self) -> Self {
        let pieces = self
            .indices()
            .map(|(x, y, z)| {
                let ox = z;
                let oz = self.size - 1 - x;
                self.pieces[self.index(ox, y, oz)]
            })
            .collect();

        return Piece {
            size: self.size,
            pieces,
        };
    }

    pub fn translation(&self, d: isize, direction: Direction) -> Option<Self> {
        if (d.abs() as usize) >= self.size || d == 0 {
            return None;
        }

        let over = if d < 0 {
            let d = -d as usize;
            self.indices()
                .filter(|(x, y, z)| match direction {
                    Direction::X => x < &d,
                    Direction::Y => y < &d,
                    Direction::Z => z < &d,
                })
                .any(|(x, y, z)| self.pieces[self.index(x, y, z)] != 0)
        } else {
            let d = self.size - (d as usize);
            self.indices()
                .filter(|(x, y, z)| match direction {
                    Direction::X => x >= &d,
                    Direction::Y => y >= &d,
                    Direction::Z => z >= &d,
                })
                .any(|(x, y, z)| self.pieces[self.index(x, y, z)] != 0)
        };

        if over {
            return None;
        }

        let pieces = self
            .indices()
            .map(|(x, y, z)| {
                let (nx, ny, nz) = match direction {
                    Direction::X => ((x as isize) - d, y as isize, z as isize),
                    Direction::Y => (x as isize, (y as isize) - d, z as isize),
                    Direction::Z => (x as isize, y as isize, (z as isize - d)),
                };

                if [nx, ny, nz]
                    .into_iter()
                    .any(|i| i < 0 || i >= (self.size as isize))
                {
                    return 0;
                }

                self.pieces[self.index(nx as _, ny as _, nz as _)]
            })
            .collect();

        Some(Piece {
            size: self.size,
            pieces,
        })
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        let s = self.size;
        Self::index_with_size(s, x, y, z)
    }

    fn index_with_size(s: usize, x: usize, y: usize, z: usize) -> usize {
        x + s * y + s * s * z
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DIMENSION {
            for z in 0..DIMENSION {
                for x in 0..DIMENSION {
                    write!(f, "{} ", self.pieces[self.index(x, y, z)])?;
                }
                write!(f, "   ")?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Piece;

    #[test]
    fn rotation() {
        let p = Piece::new(3, &[0, 1, 1, 1, 1, 0, 1, 0, 0]).unwrap();
        let mut rp = p.clone();
        for i in 0..4 {
            if i != 0 {
                assert_ne!(p, rp)
            }
            println!("{}", rp);
            rp = rp.y_rotation();
        }
        assert_eq!(p, rp);

        for i in 0..4 {
            if i != 0 {
                assert_ne!(p, rp)
            }
            rp = rp.z_rotation();
        }
        assert_eq!(p, rp);

        rp = rp.translation(1, crate::piece::Direction::Z).unwrap();
        let p = rp.clone();
        for i in 0..4 {
            if i != 0 {
                assert_ne!(p, rp)
            }
            rp = rp.y_rotation();
        }
        assert_eq!(p, rp);
    }

    #[test]
    fn translation() {
        let p = Piece::new(3, &[1, 1, 1, 1, 0, 0, 1, 0, 0]).unwrap();

        for i in -2..=2 {
            assert_eq!(p.translation(i, crate::piece::Direction::X), None);
            assert_eq!(p.translation(i, crate::piece::Direction::Y), None);
        }

        for i in -2..=0 {
            assert_eq!(p.translation(i, crate::piece::Direction::Z), None);
        }

        assert_eq!(
            p.translation(1, crate::piece::Direction::Z),
            Piece::new(3, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0])
        );
    }
}
