use crate::{
    Castling, Piece, Side,
    mv::{Mv, PromoPiece},
    position::Position,
    square::Square,
};

static KNIGHT: [(i32, i32); 8] = [
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (-1, -2),
    (1, -2),
    (-2, -1),
    (-2, 1),
];

static BISHOP: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

static ROOK: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

static QUEEN: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

static KING: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// nx/ny are always range-checked against 0..8 before being cast back to u8.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn generate_nonsliding(pos: &Position, movelist: &mut Vec<Mv>, from: Square, dirs: &[(i32, i32)]) {
    for (dx, dy) in dirs {
        let nx = i32::from(from.get_x()) + dx;
        let ny = i32::from(from.get_y()) + dy;

        // Destination square is off the board
        if !(0..8).contains(&nx) || !(0..8).contains(&ny) {
            continue;
        }

        let to = Square::from_file_rank(nx as u8, ny as u8);

        // Can't capture friendly pieces
        if pos.get_side_on(to) == Some(pos.turn) {
            continue;
        }

        movelist.push(Mv {
            from,
            to,
            promo: None,
        });
    }
}

// nx/ny are always range-checked against 0..8 before being cast back to u8.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn generate_sliding(pos: &Position, movelist: &mut Vec<Mv>, from: Square, dirs: &[(i32, i32)]) {
    for (dx, dy) in dirs {
        let mut nx = i32::from(from.get_x()) + dx;
        let mut ny = i32::from(from.get_y()) + dy;

        while (0..8).contains(&nx) && (0..8).contains(&ny) {
            let to = Square::from_file_rank(nx as u8, ny as u8);

            // Can't capture friendly pieces
            if pos.get_side_on(to) == Some(pos.turn) {
                break;
            }

            movelist.push(Mv {
                from,
                to,
                promo: None,
            });

            // Stop on a capture
            if pos.get_side_on(to) == Some(!pos.turn) {
                break;
            }

            nx += dx;
            ny += dy;
        }
    }
}

fn push_promotions(movelist: &mut Vec<Mv>, from: Square, to: Square) {
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Queen),
    });
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Rook),
    });
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Bishop),
    });
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Knight),
    });
}

impl Position {
    /// Generate pseudolegal moves
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn pseudolegal_moves(&self) -> Vec<Mv> {
        let mut movelist = vec![];

        for x in 0..8i32 {
            for y in 0..8i32 {
                let from = Square::from_file_rank(x as u8, y as u8);
                let piece = self.get_side_piece_on(from);

                if self.get_side_on(from) != Some(self.turn) {
                    continue;
                }

                match piece {
                    Some(Piece::WP | Piece::BP) => {
                        let forwards = if self.turn == Side::White { 1 } else { -1 };
                        let promo_rank = if self.turn == Side::White { 7 } else { 0 };
                        let double_rank = if self.turn == Side::White { 1 } else { 6 };
                        let single = Square::from_file_rank(x as u8, (y + forwards) as u8);
                        let is_promo = y + forwards == promo_rank;

                        // Captures
                        for dx in [-1, 1] {
                            let nx = x + dx;
                            let ny = y + forwards;

                            // Off the board
                            if !(0..=7).contains(&nx) {
                                continue;
                            }

                            let to = Square::from_file_rank(nx as u8, ny as u8);
                            let is_ep = self.ep == Some(to);
                            let is_capture = is_ep || self.get_side_on(to) == Some(!self.turn);

                            // Not a capture or EP
                            if !is_capture && !is_ep {
                                continue;
                            }

                            if is_promo {
                                push_promotions(&mut movelist, from, to);
                            } else {
                                movelist.push(Mv {
                                    from,
                                    to,
                                    promo: None,
                                });
                            }
                        }

                        // Double move
                        if y == double_rank
                            && let double =
                                Square::from_file_rank(x as u8, (y + forwards + forwards) as u8)
                            && self.get_side_on(single).is_none()
                            && self.get_side_on(double).is_none()
                        {
                            movelist.push(Mv {
                                from,
                                to: double,
                                promo: None,
                            });
                        }

                        // Single move
                        if self.get_side_on(single).is_none() {
                            if is_promo {
                                push_promotions(&mut movelist, from, single);
                            } else {
                                movelist.push(Mv {
                                    from,
                                    to: single,
                                    promo: None,
                                });
                            }
                        }
                    }
                    Some(Piece::WN | Piece::BN) => {
                        generate_nonsliding(self, &mut movelist, from, &KNIGHT);
                    }
                    Some(Piece::WB | Piece::BB) => {
                        generate_sliding(self, &mut movelist, from, &BISHOP);
                    }
                    Some(Piece::WR | Piece::BR) => {
                        generate_sliding(self, &mut movelist, from, &ROOK);
                    }
                    Some(Piece::WQ | Piece::BQ) => {
                        generate_sliding(self, &mut movelist, from, &QUEEN);
                    }
                    Some(Piece::WK | Piece::BK) => {
                        generate_nonsliding(self, &mut movelist, from, &KING);
                    }
                    None => unreachable!("Empty square already accounted for"),
                }
            }
        }

        let in_check = self.in_check();

        // Castling - white king side
        if self.turn == Side::White
            && self.castling[Castling::WKS]
            && !in_check
            && self.get_side_piece_on(Square::F1).is_none()
            && self.get_side_piece_on(Square::G1).is_none()
            && !self.is_attacked(Square::F1, !self.turn)
            && !self.is_attacked(Square::G1, !self.turn)
        {
            movelist.push(Mv {
                from: Square::E1,
                to: Square::G1,
                promo: None,
            });
        }

        // Castling - white queen side
        if self.turn == Side::White
            && self.castling[Castling::WQS]
            && !in_check
            && self.get_side_piece_on(Square::D1).is_none()
            && self.get_side_piece_on(Square::C1).is_none()
            && self.get_side_piece_on(Square::B1).is_none()
            && !self.is_attacked(Square::D1, !self.turn)
            && !self.is_attacked(Square::C1, !self.turn)
        {
            movelist.push(Mv {
                from: Square::E1,
                to: Square::C1,
                promo: None,
            });
        }

        // Castling - black king side
        if self.turn == Side::Black
            && self.castling[Castling::BKS]
            && !in_check
            && self.get_side_piece_on(Square::F8).is_none()
            && self.get_side_piece_on(Square::G8).is_none()
            && !self.is_attacked(Square::F8, !self.turn)
            && !self.is_attacked(Square::G8, !self.turn)
        {
            movelist.push(Mv {
                from: Square::E8,
                to: Square::G8,
                promo: None,
            });
        }

        // Castling - black queen side
        if self.turn == Side::Black
            && self.castling[Castling::BQS]
            && !in_check
            && self.get_side_piece_on(Square::D8).is_none()
            && self.get_side_piece_on(Square::C8).is_none()
            && self.get_side_piece_on(Square::B8).is_none()
            && !self.is_attacked(Square::D8, !self.turn)
            && !self.is_attacked(Square::C8, !self.turn)
        {
            movelist.push(Mv {
                from: Square::E8,
                to: Square::C8,
                promo: None,
            });
        }

        movelist
    }
}
