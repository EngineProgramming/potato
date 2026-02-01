use crate::{Piece, Side, position::Position, square::Square};

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

impl Position {
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn is_attacked(&self, sq: Square, side: Side) -> bool {
        let knight = if side == Side::White {
            Piece::WN
        } else {
            Piece::BN
        };

        let bishop = if side == Side::White {
            Piece::WB
        } else {
            Piece::BB
        };

        let rook = if side == Side::White {
            Piece::WR
        } else {
            Piece::BR
        };

        let queen = if side == Side::White {
            Piece::WQ
        } else {
            Piece::BQ
        };

        // Pawns
        if side == Side::White {
            if sq.get_x() < 7 && sq.get_y() > 0 {
                let nsq = Square::from_file_rank(sq.get_x() + 1, sq.get_y() - 1);
                if self.get_side_piece_on(nsq) == Some(Piece::WP) {
                    return true;
                }
            }
            if sq.get_x() > 0 && sq.get_y() > 0 {
                let nsq = Square::from_file_rank(sq.get_x() - 1, sq.get_y() - 1);
                if self.get_side_piece_on(nsq) == Some(Piece::WP) {
                    return true;
                }
            }
        } else {
            if sq.get_x() < 7 && sq.get_y() < 7 {
                let nsq = Square::from_file_rank(sq.get_x() + 1, sq.get_y() + 1);
                if self.get_side_piece_on(nsq) == Some(Piece::BP) {
                    return true;
                }
            }
            if sq.get_x() > 0 && sq.get_y() < 7 {
                let nsq = Square::from_file_rank(sq.get_x() - 1, sq.get_y() + 1);
                if self.get_side_piece_on(nsq) == Some(Piece::BP) {
                    return true;
                }
            }
        }

        // Knights
        for (dx, dy) in KNIGHT {
            let nx = i32::from(sq.get_x()) + dx;
            let ny = i32::from(sq.get_y()) + dy;

            if !(0..=7).contains(&nx) || !(0..=7).contains(&ny) {
                continue;
            }

            let nsq = Square::from_file_rank(nx as u8, ny as u8);
            if self.get_side_piece_on(nsq) == Some(knight) {
                return true;
            }
        }

        // Bishop/Queen
        for (dx, dy) in BISHOP {
            let mut nx = i32::from(sq.get_x()) + dx;
            let mut ny = i32::from(sq.get_y()) + dy;

            while (0..8).contains(&nx) && (0..8).contains(&ny) {
                let nsq = Square::from_file_rank(nx as u8, ny as u8);

                if let Some(piece) = self.get_side_piece_on(nsq) {
                    if piece == bishop || piece == queen {
                        return true;
                    }
                    break;
                }

                nx += dx;
                ny += dy;
            }
        }

        // Rook/Queen
        for (dx, dy) in ROOK {
            let mut nx = i32::from(sq.get_x()) + dx;
            let mut ny = i32::from(sq.get_y()) + dy;

            while (0..8).contains(&nx) && (0..8).contains(&ny) {
                let nsq = Square::from_file_rank(nx as u8, ny as u8);

                if let Some(piece) = self.get_side_piece_on(nsq) {
                    if piece == rook || piece == queen {
                        return true;
                    }
                    break;
                }

                nx += dx;
                ny += dy;
            }
        }

        // King
        let ksq = self.ksq[side].expect("King not found");
        let dx = i32::from(sq.get_x()) - i32::from(ksq.get_x());
        let dy = i32::from(sq.get_y()) - i32::from(ksq.get_y());
        if (-1..=1).contains(&dx) && (-1..=1).contains(&dy) {
            return true;
        }

        false
    }

    /// Is the side to move's king currently in check?
    #[must_use]
    pub fn in_check(&self) -> bool {
        let ksq = self.ksq[self.turn].expect("King not found");
        self.is_attacked(ksq, !self.turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_attacked() {
        let pos = Position::from_fen("4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1").unwrap();

        // Attacked
        let attacked = ["d1", "e2", "f2", "f3", "g7", "f8"];
        for sqstr in attacked {
            let sq: Square = sqstr.parse().unwrap();
            assert!(pos.is_attacked(sq, Side::Black), "{}", sq);
        }

        // Not attacked
        let not_attacked = ["a2", "a4", "a8", "b8", "d5", "g1", "e1"];
        for sqstr in not_attacked {
            let sq: Square = sqstr.parse().unwrap();
            assert!(!pos.is_attacked(sq, Side::Black), "{}", sq);
        }
    }

    #[test]
    fn in_check_true() {
        let pos = Position::from_fen("4k3/8/8/8/8/8/4r3/4K3 w - - 0 1").unwrap();
        assert!(pos.in_check());
    }

    #[test]
    fn in_check_false() {
        let pos = Position::from_fen("startpos").unwrap();
        assert!(!pos.in_check());
    }
}
