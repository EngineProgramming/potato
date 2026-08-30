use crate::{
    Side,
    position::Position,
    zobrist::{SIDE_KEY, ep_key},
};

impl Position {
    /// Make a null move - pass the turn to the opponent without moving a piece.
    /// Can be undone with `undomove`.
    pub fn makenull<const UPDATE_HASH: bool>(&mut self) {
        self.history.push(self.save_undo());

        self.halfmoves = 0;

        // Clear EP square
        if UPDATE_HASH && let Some(sq) = self.ep {
            self.hash ^= ep_key(sq.get_x());
        }
        self.ep = None;

        // Fullmove counter increments after Black's move
        if self.turn == Side::Black {
            self.fullmoves += 1;
        }

        // Side to move
        self.turn = !self.turn;

        if UPDATE_HASH {
            self.hash ^= SIDE_KEY;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makenull_flips_turn() {
        let mut pos = Position::from_fen("startpos").unwrap();
        assert_eq!(pos.turn, Side::White);

        pos.makenull::<true>();

        assert_eq!(pos.turn, Side::Black);
    }

    #[test]
    fn makenull_clears_ep_square() {
        let fen = "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq d6 0 1";
        let mut pos = Position::from_fen(fen).unwrap();
        assert!(pos.ep.is_some());

        pos.makenull::<true>();

        assert!(pos.ep.is_none());
    }

    #[test]
    fn makenull_undomove_restores_position() {
        let fen = "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq d6 0 1";
        let mut pos = Position::from_fen(fen).unwrap();
        let before = pos.get_fen();
        let before_hash = pos.hash;

        pos.makenull::<true>();
        pos.undomove();

        assert_eq!(pos.get_fen(), before);
        assert_eq!(pos.hash, before_hash);
    }

    #[test]
    fn makenull_resets_halfmoves() {
        let fen = "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq - 12 1";
        let mut pos = Position::from_fen(fen).unwrap();
        assert_eq!(pos.halfmoves, 12);

        pos.makenull::<true>();

        assert_eq!(pos.halfmoves, 0);
    }

    #[test]
    fn double_makenull_restores_position() {
        let fen = "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq - 12 5";
        let mut pos = Position::from_fen(fen).unwrap();
        let before_hash = pos.hash;

        pos.makenull::<true>();
        pos.makenull::<true>();

        assert_eq!(pos.get_fen(), "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq - 0 6");
        assert_eq!(pos.hash, before_hash);
    }

    #[test]
    fn makenull_increments_fullmoves_for_black() {
        let mut pos = Position::from_fen("startpos").unwrap();
        let start = pos.fullmoves;
        pos.makenull::<true>();
        assert_eq!(
            pos.fullmoves, start,
            "fullmoves shouldn't increment after White's null move"
        );

        pos.makenull::<true>();
        assert_eq!(
            pos.fullmoves,
            start + 1,
            "fullmoves should increment after Black's null move"
        );
    }
}
