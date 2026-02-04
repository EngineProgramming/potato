use crate::{
    Castling, Piece, Side,
    mv::{Mv, PromoPiece},
    position::Position,
    square::Square,
};

impl Position {
    /// Apply a pseudolegal move to the board
    /// Returns whether the move was legal or not
    #[must_use]
    pub fn makemove(&mut self, mv: &Mv) -> bool {
        let piece = self
            .get_piece_on(mv.from)
            .expect("No piece on move origin square");

        let captured = self.get_piece_on(mv.to);

        self.halfmoves += 1;

        // Remove piece
        self.clear_square(mv.from);

        // Remove captured
        self.clear_square(mv.to);

        // Place piece
        self.set_piece(mv.to, self.turn, piece);

        // Pawn moves reset the halfmove counter
        if piece == Piece::Pawn {
            self.halfmoves = 0;
        }

        // Captures reset the halfmove counter
        if captured.is_some() {
            self.halfmoves = 0;
        }

        // Did we just capture with EP?
        if let Some(sq) = self.ep
            && piece == Piece::Pawn
            && mv.to == sq
        {
            if self.turn == Side::White {
                self.clear_square(Square::from_file_rank(mv.to.x, mv.to.y - 1));
            } else {
                self.clear_square(Square::from_file_rank(mv.to.x, mv.to.y + 1));
            }

            self.halfmoves = 0;
        }

        // Clear EP square
        self.ep = None;

        // Set EP square?
        if piece == Piece::Pawn && mv.from.y == 1 && mv.to.y == 3 {
            self.ep = Some(Square::from_file_rank(mv.from.x, 2));
        }
        if piece == Piece::Pawn && mv.from.y == 6 && mv.to.y == 4 {
            self.ep = Some(Square::from_file_rank(mv.from.x, 5));
        }

        // Promotions
        match mv.promo {
            Some(PromoPiece::Queen) => {
                self.pieces[Piece::Pawn as usize].unset(mv.to);
                self.pieces[Piece::Queen as usize].set(mv.to);
            }
            Some(PromoPiece::Rook) => {
                self.pieces[Piece::Pawn as usize].unset(mv.to);
                self.pieces[Piece::Rook as usize].set(mv.to);
            }
            Some(PromoPiece::Bishop) => {
                self.pieces[Piece::Pawn as usize].unset(mv.to);
                self.pieces[Piece::Bishop as usize].set(mv.to);
            }
            Some(PromoPiece::Knight) => {
                self.pieces[Piece::Pawn as usize].unset(mv.to);
                self.pieces[Piece::Knight as usize].set(mv.to);
            }
            None => {}
        }

        // Castling permissions - Did white's king rook move or get captured?
        if mv.from == Square::from_index(7) || mv.to == Square::from_index(7) {
            self.castling[Castling::WKS as usize] = false;
        }

        // Castling permissions - Did white's queen rook move or get captured?
        if mv.from == Square::from_index(0) || mv.to == Square::from_index(0) {
            self.castling[Castling::WQS as usize] = false;
        }

        // Castling permissions - Did white's king move?
        if mv.from == Square::from_index(4) {
            self.castling[Castling::WKS as usize] = false;
            self.castling[Castling::WQS as usize] = false;
        }

        // Castling permissions - Did black's king rook move or get captured?
        if mv.from == Square::from_index(63) || mv.to == Square::from_index(63) {
            self.castling[Castling::BKS as usize] = false;
        }

        // Castling permissions - Did black's queen rook move or get captured?
        if mv.from == Square::from_index(56) || mv.to == Square::from_index(56) {
            self.castling[Castling::BQS as usize] = false;
        }

        // Castling permissions - Did black's king move?
        if mv.from == Square::from_index(60) {
            self.castling[Castling::BKS as usize] = false;
            self.castling[Castling::BQS as usize] = false;
        }

        // Castling wks
        if piece == Piece::King
            && mv.from == Square::from_index(4)
            && mv.to == Square::from_index(6)
        {
            self.clear_square(Square::from_index(7));
            self.set_piece(Square::from_index(5), Side::White, Piece::Rook);
        }

        // Castling wqs
        if piece == Piece::King
            && mv.from == Square::from_index(4)
            && mv.to == Square::from_index(2)
        {
            self.clear_square(Square::from_index(0));
            self.set_piece(Square::from_index(3), Side::White, Piece::Rook);
        }

        // Castling bks
        if piece == Piece::King
            && mv.from == Square::from_index(60)
            && mv.to == Square::from_index(62)
        {
            self.clear_square(Square::from_index(63));
            self.set_piece(Square::from_index(61), Side::Black, Piece::Rook);
        }

        // Castling bqs
        if piece == Piece::King
            && mv.from == Square::from_index(60)
            && mv.to == Square::from_index(58)
        {
            self.clear_square(Square::from_index(56));
            self.set_piece(Square::from_index(59), Side::Black, Piece::Rook);
        }

        // Side to move
        self.turn = !self.turn;

        // Legality check
        let ksq = (self.colours[!self.turn as usize] & self.pieces[Piece::King as usize]).pop_lsb();
        !self.is_attacked(ksq, self.turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_makemove_legal() {
        let fen = "4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1";
        let legal = ["e3e2", "e3e7", "f5g7", "b7b8q", "b7b8n"];

        for movestr in legal {
            let mut pos = Position::from_fen(fen);
            let mv = Mv::from_string(movestr);
            let success = pos.makemove(&mv);
            assert!(success, "Move \"{movestr}\" was meant to be legal");
        }
    }

    #[test]
    fn test_makemove_illegal() {
        let fen = "4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1";
        let illegal = ["e1d1", "e1d2", "e1e2", "e1f2", "e1f1", "e3d2", "e3b6"];

        for movestr in illegal {
            let mut pos = Position::from_fen(fen);
            let mv = Mv::from_string(movestr);
            let success = pos.makemove(&mv);
            assert!(!success, "Move \"{movestr}\" was meant to be illegal");
        }
    }

    #[test]
    fn test_changes_white() {
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
            let mut pos = Position::from_fen(startfen);
            let mv = Mv::from_string(movestr);
            let success = pos.makemove(&mv);

            assert!(success);
            assert_eq!(pos.get_fen(), fen);
        }
    }
}
