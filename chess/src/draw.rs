use crate::position::Position;

impl Position {
    /// Draw by the 50 move rule
    #[must_use]
    pub const fn is_fifty_move_draw(&self) -> bool {
        self.halfmoves >= 100
    }

    /// Draw by threefold
    #[must_use]
    pub fn is_threefold_repetition(&self) -> bool {
        if self.history.len() < 8 {
            return false;
        }

        let limit = self.halfmoves as usize;
        let mut count = 1;

        for undo in self.history.iter().rev().take(limit).skip(1).step_by(2) {
            if undo.hash == self.hash {
                count += 1;

                if count >= 3 {
                    return true;
                }
            }
        }

        false
    }

    /// Is the position a draw?
    #[must_use]
    pub fn is_draw(&self) -> bool {
        self.is_fifty_move_draw() || self.is_threefold_repetition()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifty_move_rule() {
        let mut pos = Position::from_fen("startpos").unwrap();
        pos.halfmoves = 99;
        assert!(!pos.is_fifty_move_draw());
        assert!(!pos.is_draw());
        pos.halfmoves = 100;
        assert!(pos.is_fifty_move_draw());
        assert!(pos.is_draw());
    }

    #[test]
    fn threefold_repetition() {
        let mut pos = Position::from_fen("startpos").unwrap();

        assert!(!pos.is_threefold_repetition());

        let moves = [
            "g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1", "f6g8",
        ];

        for movestr in moves {
            assert!(!pos.is_threefold_repetition());
            assert!(!pos.is_draw());

            let mv = movestr.parse().unwrap();
            assert!(pos.makemove::<true>(&mv));
        }

        assert!(pos.is_threefold_repetition());
        assert!(pos.is_draw());
    }

    #[test]
    fn halfmoves_reset_by_pawn_move() {
        let mut pos = Position::from_fen("startpos").unwrap();

        assert!(pos.makemove::<true>(&"e2e4".parse().unwrap()));
        assert_eq!(pos.halfmoves, 0);
        assert!(!pos.is_threefold_repetition());
        assert!(!pos.is_draw());
    }
}
