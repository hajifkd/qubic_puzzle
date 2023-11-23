use crate::piece::Piece;

mod piece;

fn compose_all(current: &Piece, rest: &[Piece]) -> bool {
    if rest.len() == 0 {
        return true;
    }

    let mut piece = rest[0].clone();
    let rest = &rest[1..];

    for rot_face in 0..6 {
        for _rot_z in 0..4 {
            for tx in -2..=2 {
                let piece = if tx == 0 {
                    piece.clone()
                } else {
                    let o = piece.translation(tx, piece::Direction::X);
                    if let Some(n) = o {
                        n
                    } else {
                        continue;
                    }
                };
                for ty in -2..=2 {
                    let piece = if ty == 0 {
                        piece.clone()
                    } else {
                        let o = piece.translation(ty, piece::Direction::Y);
                        if let Some(n) = o {
                            n
                        } else {
                            continue;
                        }
                    };
                    for tz in -2..=2 {
                        let piece = if tz == 0 {
                            piece.clone()
                        } else {
                            let o = piece.translation(tz, piece::Direction::Z);
                            if let Some(n) = o {
                                n
                            } else {
                                continue;
                            }
                        };

                        let new = current.append(&piece);

                        if let Some(new) = new {
                            if compose_all(&new, rest) {
                                println!("{}", piece);
                                return true;
                            }
                        }
                    }
                }
            }

            piece = piece.z_rotation();
        }
        if rot_face < 3 {
            piece = piece.y_rotation();
        } else if rot_face == 3 {
            piece = piece.y_rotation().z_rotation().y_rotation();
        } else if rot_face == 4 {
            piece = piece.y_rotation().y_rotation();
        }
    }

    false
}

fn main() {
    let size: usize = 3;
    let pss: Vec<_> = [
        vec![0, 1, 1, 1, 1, 0, 1, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 0, 0, 0],
    ]
    .into_iter()
    .map(|ps| Piece::new(size, &ps).unwrap())
    .collect();
    let result = compose_all(&Piece::new(3, &[]).unwrap(), &pss);

    println!("{}", result);
}
