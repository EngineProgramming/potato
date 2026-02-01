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

fn generate_nonsliding(pos: &Position, movelist: &mut Vec<Mv>, from: Square, dirs: &[(i32, i32)]) {
    for (dx, dy) in dirs {
        let nx = from.x as i32 + dx;
        let ny = from.y as i32 + dy;

        // Destination square is off the board
        if !(0..=7).contains(&nx) || !(0..=7).contains(&ny) {
            continue;
        }

        let to = Square::from_file_rank(nx as u8, ny as u8);

        // Capture friendly piece
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

fn generate_sliding(pos: &Position, movelist: &mut Vec<Mv>, from: Square, dirs: &[(i32, i32)]) {
    for (dx, dy) in dirs {
        let mut nx = from.x as i32 + dx;
        let mut ny = from.y as i32 + dy;

        while (0..8).contains(&nx) && (0..8).contains(&ny) {
            let to = Square::from_file_rank(nx as u8, ny as u8);
            if pos.get_side_on(to) == Some(pos.turn) {
                break;
            }
            movelist.push(Mv {
                from,
                to,
                promo: None,
            });
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
    /// This includes moves that leave the king in check
    #[must_use]
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
                    Some(Piece::WP) | Some(Piece::BP) => {
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
                    Some(Piece::WN) | Some(Piece::BN) => {
                        generate_nonsliding(self, &mut movelist, from, &KNIGHT)
                    }
                    Some(Piece::WB) | Some(Piece::BB) => {
                        generate_sliding(self, &mut movelist, from, &BISHOP)
                    }
                    Some(Piece::WR) | Some(Piece::BR) => {
                        generate_sliding(self, &mut movelist, from, &ROOK)
                    }
                    Some(Piece::WQ) | Some(Piece::BQ) => {
                        generate_sliding(self, &mut movelist, from, &QUEEN)
                    }
                    Some(Piece::WK) | Some(Piece::BK) => {
                        generate_nonsliding(self, &mut movelist, from, &KING)
                    }
                    None => unreachable!("Empty square already accounted for"),
                }
            }
        }

        let in_check = self.is_attacked(
            self.ksq[self.turn as usize].expect("ksq not found"),
            !self.turn,
        );

        // Castling - white king side
        if self.turn == Side::White
            && self.castling[Castling::WKS as usize]
            && !in_check
            && self.get_side_piece_on(Square::from_index(5)).is_none()
            && self.get_side_piece_on(Square::from_index(6)).is_none()
            && !self.is_attacked(Square::from_index(5), !self.turn)
            && !self.is_attacked(Square::from_index(6), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(4),
                to: Square::from_index(6),
                promo: None,
            });
        }

        // Castling - white queen side
        if self.turn == Side::White
            && self.castling[Castling::WQS as usize]
            && !in_check
            && self.get_side_piece_on(Square::from_index(3)).is_none()
            && self.get_side_piece_on(Square::from_index(2)).is_none()
            && self.get_side_piece_on(Square::from_index(1)).is_none()
            && !self.is_attacked(Square::from_index(3), !self.turn)
            && !self.is_attacked(Square::from_index(2), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(4),
                to: Square::from_index(2),
                promo: None,
            });
        }

        // Castling - black king side
        if self.turn == Side::Black
            && self.castling[Castling::BKS as usize]
            && !in_check
            && self.get_side_piece_on(Square::from_index(61)).is_none()
            && self.get_side_piece_on(Square::from_index(62)).is_none()
            && !self.is_attacked(Square::from_index(61), !self.turn)
            && !self.is_attacked(Square::from_index(62), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(60),
                to: Square::from_index(62),
                promo: None,
            });
        }

        // Castling - black queen side
        if self.turn == Side::Black
            && self.castling[Castling::BQS as usize]
            && !in_check
            && self.get_side_piece_on(Square::from_index(59)).is_none()
            && self.get_side_piece_on(Square::from_index(58)).is_none()
            && self.get_side_piece_on(Square::from_index(57)).is_none()
            && !self.is_attacked(Square::from_index(59), !self.turn)
            && !self.is_attacked(Square::from_index(58), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(60),
                to: Square::from_index(58),
                promo: None,
            });
        }

        movelist
    }
}
