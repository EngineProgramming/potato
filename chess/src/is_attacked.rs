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
    /// Is the square attacked?
    #[must_use]
    pub fn is_attacked(&self, sq: Square, side: Side) -> bool {
        match side {
            Side::White => self.is_attacked_white(sq),
            Side::Black => self.is_attacked_black(sq),
        }
    }

    /// Is the square attacked by white?
    #[must_use]
    fn is_attacked_white(&self, sq: Square) -> bool {
        // Pawns
        if sq.x < 7 && sq.y > 0 {
            let nsq = Square::from_file_rank(sq.x + 1, sq.y - 1);
            if self.get_side_piece_on(nsq) == Some(Piece::WP) {
                return true;
            }
        }
        if sq.x > 0 && sq.y > 0 {
            let nsq = Square::from_file_rank(sq.x - 1, sq.y - 1);
            if self.get_side_piece_on(nsq) == Some(Piece::WP) {
                return true;
            }
        }

        // Knights
        {
            for (dx, dy) in KNIGHT {
                let nx = sq.x as i32 + dx;
                let ny = sq.y as i32 + dy;

                if !(0..=7).contains(&nx) || !(0..=7).contains(&ny) {
                    continue;
                }

                let nsq = Square::from_file_rank(nx as u8, ny as u8);
                if self.get_side_piece_on(nsq) == Some(Piece::WN) {
                    return true;
                }
            }
        }

        // Bishop/Queen
        for (dx, dy) in BISHOP {
            let mut nx = sq.x as i32 + dx;
            let mut ny = sq.y as i32 + dy;

            while (0..8).contains(&nx) && (0..8).contains(&ny) {
                let nsq = Square::from_file_rank(nx as u8, ny as u8);
                match self.get_side_piece_on(nsq) {
                    Some(Piece::WQ) => return true,
                    Some(Piece::WB) => return true,
                    Some(_) => break,
                    None => {}
                }
                nx += dx;
                ny += dy;
            }
        }

        // Rook/Queen
        for (dx, dy) in ROOK {
            let mut nx = sq.x as i32 + dx;
            let mut ny = sq.y as i32 + dy;

            while (0..8).contains(&nx) && (0..8).contains(&ny) {
                let nsq = Square::from_file_rank(nx as u8, ny as u8);
                match self.get_side_piece_on(nsq) {
                    Some(Piece::WQ) => return true,
                    Some(Piece::WR) => return true,
                    Some(_) => break,
                    None => {}
                }
                nx += dx;
                ny += dy;
            }
        }

        // King
        {
            let dx = sq.x as i32 - self.ksq[Side::White as usize].expect("King not found").x as i32;
            let dy = sq.y as i32 - self.ksq[Side::White as usize].expect("King not found").y as i32;
            if (-1..=1).contains(&dx) && (-1..=1).contains(&dy) {
                return true;
            }
        }

        false
    }

    /// Is the square attacked by black?
    #[must_use]
    fn is_attacked_black(&self, sq: Square) -> bool {
        // Pawns
        if sq.x < 7 && sq.y < 7 {
            let nsq = Square::from_file_rank(sq.x + 1, sq.y + 1);
            if self.get_side_piece_on(nsq) == Some(Piece::BP) {
                return true;
            }
        }
        if sq.x > 0 && sq.y < 7 {
            let nsq = Square::from_file_rank(sq.x - 1, sq.y + 1);
            if self.get_side_piece_on(nsq) == Some(Piece::BP) {
                return true;
            }
        }

        // Knights
        {
            for (dx, dy) in KNIGHT {
                let nx = sq.x as i32 + dx;
                let ny = sq.y as i32 + dy;

                if !(0..=7).contains(&nx) || !(0..=7).contains(&ny) {
                    continue;
                }

                let nsq = Square::from_file_rank(nx as u8, ny as u8);
                if self.get_side_piece_on(nsq) == Some(Piece::BN) {
                    return true;
                }
            }
        }

        // Bishop/Queen
        for (dx, dy) in BISHOP {
            let mut nx = sq.x as i32 + dx;
            let mut ny = sq.y as i32 + dy;

            while (0..8).contains(&nx) && (0..8).contains(&ny) {
                let nsq = Square::from_file_rank(nx as u8, ny as u8);
                match self.get_side_piece_on(nsq) {
                    Some(Piece::BQ) => return true,
                    Some(Piece::BB) => return true,
                    Some(_) => break,
                    None => {}
                }
                nx += dx;
                ny += dy;
            }
        }

        // Rook/Queen
        for (dx, dy) in ROOK {
            let mut nx = sq.x as i32 + dx;
            let mut ny = sq.y as i32 + dy;

            while (0..8).contains(&nx) && (0..8).contains(&ny) {
                let nsq = Square::from_file_rank(nx as u8, ny as u8);
                match self.get_side_piece_on(nsq) {
                    Some(Piece::BQ) => return true,
                    Some(Piece::BR) => return true,
                    Some(_) => break,
                    None => {}
                }
                nx += dx;
                ny += dy;
            }
        }

        // King
        {
            let dx = sq.x as i32 - self.ksq[Side::Black as usize].expect("King not found").x as i32;
            let dy = sq.y as i32 - self.ksq[Side::Black as usize].expect("King not found").y as i32;
            if (-1..=1).contains(&dx) && (-1..=1).contains(&dy) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_attacked_black() {
        let pos = Position::from_fen("4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1");

        // Attacked
        let attacked = ["d1", "e2", "f2", "f3", "g7", "f8"];
        for sqstr in attacked {
            let sq = Square::from_string(sqstr);
            assert!(pos.is_attacked_black(sq), "{}", sq);
        }

        // Not attacked
        let not_attacked = ["a2", "a4", "a8", "b8", "d5", "g1", "e1"];
        for sqstr in not_attacked {
            let sq = Square::from_string(sqstr);
            assert!(!pos.is_attacked_black(sq), "{}", sq);
        }
    }
}
