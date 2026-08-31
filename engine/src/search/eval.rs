use chess::{Piece, Side, position::Position, square::Square};

/// Sum the material on the board from the perspective of `side`
#[must_use]
pub fn eval(pos: &Position) -> i32 {
    let mut score = 0;

    for y in 0..8 {
        for x in 0..8 {
            let sq = Square::from_file_rank(x, y);

            // Material
            score += match pos.get_side_piece_on(sq) {
                Some(Piece::WP) => 100,
                Some(Piece::BP) => -100,
                Some(Piece::WN) => 300,
                Some(Piece::BN) => -300,
                Some(Piece::WB) => 320,
                Some(Piece::BB) => -320,
                Some(Piece::WR) => 500,
                Some(Piece::BR) => -500,
                Some(Piece::WQ) => 900,
                Some(Piece::BQ) => -900,
                Some(Piece::WK) => 0,
                Some(Piece::BK) => 0,
                None => 0,
            };
        }
    }

    // Side to move relative
    match pos.turn {
        Side::White => score,
        Side::Black => -score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startpos_is_balanced() {
        let pos = Position::from_fen("startpos").unwrap();
        assert_eq!(eval(&pos), 0);
    }

    #[test]
    fn white_up_a_queen_is_high_for_white_to_move() {
        let pos = Position::from_fen("4k3/8/8/8/8/8/8/Q3K3 w - - 0 1").unwrap();
        assert!(eval(&pos) > 800);
    }

    #[test]
    fn white_up_a_queen_is_low_for_black_to_move() {
        let pos = Position::from_fen("4k3/8/8/8/8/8/8/Q3K3 b - - 0 1").unwrap();
        assert!(eval(&pos) < -800);
    }

    #[test]
    fn black_up_a_queen_is_high_for_black_to_move() {
        let pos = Position::from_fen("4kq2/8/8/8/8/8/8/4K3 b - - 0 1").unwrap();
        assert!(eval(&pos) > 800);
    }

    #[test]
    fn black_up_a_queen_is_low_for_white_to_move() {
        let pos = Position::from_fen("4kq2/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert!(eval(&pos) < -800);
    }

    #[test]
    fn white_up_all_the_pieces_is_very_high() {
        let pos = Position::from_fen("4k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1").unwrap();
        assert!(eval(&pos) > 3500);
    }
}
