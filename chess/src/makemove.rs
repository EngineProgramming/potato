use crate::{
    Piece, Side,
    mv::{Mv, PromoPiece},
    position::Position,
    square::Square,
};

impl Position {
    #[must_use]
    pub fn makemove(&mut self, mv: &Mv) -> bool {
        let piece = self
            .get_side_piece_on(mv.from)
            .expect("No piece on move origin square");

        let captured = self.get_side_piece_on(mv.to);

        self.halfmoves += 1;

        // Remove piece
        self.clear_square(mv.from);

        // Remove captured
        self.set_piece(piece, mv.to);

        // Pawn moves reset the halfmove counter
        if piece == Piece::WP || piece == Piece::BP {
            self.halfmoves = 0;
        }

        // Captures reset the halfmove counter
        if captured.is_some() {
            self.halfmoves = 0;
        }

        // Did we just capture with EP?
        if let Some(sq) = self.ep
            && piece == Piece::WP
            && mv.to == sq
        {
            self.clear_square(Square::from_file_rank(mv.to.get_x(), mv.to.get_y() - 1));
            self.halfmoves = 0;
        }
        if let Some(sq) = self.ep
            && piece == Piece::BP
            && mv.to == sq
        {
            self.clear_square(Square::from_file_rank(mv.to.get_x(), mv.to.get_y() + 1));
            self.halfmoves = 0;
        }

        // Clear EP square
        self.ep = None;

        // Set EP square?
        if piece == Piece::WP && mv.from.get_y() == 1 && mv.to.get_y() == 3 {
            self.ep = Some(Square::from_file_rank(mv.from.get_x(), 2));
        }
        if piece == Piece::BP && mv.from.get_y() == 6 && mv.to.get_y() == 4 {
            self.ep = Some(Square::from_file_rank(mv.from.get_x(), 5));
        }

        // Promotions
        match (self.turn, mv.promo) {
            (Side::White, Some(PromoPiece::Queen)) => self.set_piece(Piece::WQ, mv.to),
            (Side::White, Some(PromoPiece::Rook)) => self.set_piece(Piece::WR, mv.to),
            (Side::White, Some(PromoPiece::Bishop)) => self.set_piece(Piece::WB, mv.to),
            (Side::White, Some(PromoPiece::Knight)) => self.set_piece(Piece::WN, mv.to),
            (Side::Black, Some(PromoPiece::Queen)) => self.set_piece(Piece::BQ, mv.to),
            (Side::Black, Some(PromoPiece::Rook)) => self.set_piece(Piece::BR, mv.to),
            (Side::Black, Some(PromoPiece::Bishop)) => self.set_piece(Piece::BB, mv.to),
            (Side::Black, Some(PromoPiece::Knight)) => self.set_piece(Piece::BN, mv.to),
            (_, _) => {}
        }

        // Castling permissions - Did white's king rook move or get captured?
        if mv.from == Square(7) || mv.to == Square(7) {
            self.castling[0] = false;
        }

        // Castling permissions - Did white's queen rook move or get captured?
        if mv.from == Square(0) || mv.to == Square(0) {
            self.castling[1] = false;
        }

        // Castling permissions - Did white's king move?
        if mv.from == Square(4) {
            self.castling[0] = false;
            self.castling[1] = false;
        }

        // Castling permissions - Did black's king rook move or get captured?
        if mv.from == Square(63) || mv.to == Square(63) {
            self.castling[2] = false;
        }

        // Castling permissions - Did black's queen rook move or get captured?
        if mv.from == Square(56) || mv.to == Square(56) {
            self.castling[3] = false;
        }

        // Castling permissions - Did black's king move?
        if mv.from == Square(60) {
            self.castling[2] = false;
            self.castling[3] = false;
        }

        // Castling wks
        if piece == Piece::WK && mv.from == Square(4) && mv.to == Square(6) {
            self.clear_square(Square(7));
            self.set_piece(Piece::WR, Square(5));
        }

        // Castling wqs
        if piece == Piece::WK && mv.from == Square(4) && mv.to == Square(2) {
            self.clear_square(Square(0));
            self.set_piece(Piece::WR, Square(3));
        }

        // Side to move
        self.turn = !self.turn;

        // Legality check

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let startfen = "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq d6 0 1";
        let tests = [
            ("e5e6", "r3k2r/6P1/4P3/3p4/8/8/4P3/R3K2R b KQkq - 0 1"),
            ("e2e3", "r3k2r/6P1/8/3pP3/8/4P3/8/R3K2R b KQkq - 0 1"),
            // Update EP square
            ("e2e4", "r3k2r/6P1/8/3pP3/4P3/8/8/R3K2R b KQkq e3 0 1"),
            ("e5d6", "r3k2r/6P1/3P4/8/8/8/4P3/R3K2R b KQkq - 0 1"),
            // Promotions
            ("g7g8q", "r3k1Qr/8/8/3pP3/8/8/4P3/R3K2R b KQkq - 0 1"),
            ("g7g8r", "r3k1Rr/8/8/3pP3/8/8/4P3/R3K2R b KQkq - 0 1"),
            ("g7g8b", "r3k1Br/8/8/3pP3/8/8/4P3/R3K2R b KQkq - 0 1"),
            ("g7g8n", "r3k1Nr/8/8/3pP3/8/8/4P3/R3K2R b KQkq - 0 1"),
            // Promotions with capture
            ("g7h8q", "r3k2Q/8/8/3pP3/8/8/4P3/R3K2R b KQq - 0 1"),
            ("g7h8r", "r3k2R/8/8/3pP3/8/8/4P3/R3K2R b KQq - 0 1"),
            ("g7h8b", "r3k2B/8/8/3pP3/8/8/4P3/R3K2R b KQq - 0 1"),
            ("g7h8n", "r3k2N/8/8/3pP3/8/8/4P3/R3K2R b KQq - 0 1"),
            // Update castling permissions
            ("e1f1", "r3k2r/6P1/8/3pP3/8/8/4P3/R4K1R b kq - 1 1"),
            ("e1g1", "r3k2r/6P1/8/3pP3/8/8/4P3/R4RK1 b kq - 1 1"),
            ("e1c1", "r3k2r/6P1/8/3pP3/8/8/4P3/2KR3R b kq - 1 1"),
            ("h1h8", "r3k2R/6P1/8/3pP3/8/8/4P3/R3K3 b Qq - 0 1"),
            ("a1a8", "R3k2r/6P1/8/3pP3/8/8/4P3/4K2R b Kk - 0 1"),
        ];

        for (movestr, fen) in tests {
            let mut pos = Position::from_fen(startfen).expect("Failed to parse fen");
            let mv = Mv::from_string(movestr).expect("Failed to parse move string");
            let success = pos.makemove(&mv);

            assert!(success);
            assert_eq!(pos.get_fen(), fen);
        }
    }
}
