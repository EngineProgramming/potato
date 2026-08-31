use crate::{
    Castling, Piece, Side,
    mv::{Mv, PromoPiece},
    position::Position,
    square::Square,
    zobrist::{SIDE_KEY, castling_key, ep_key},
};

impl Position {
    /// Apply a pseudolegal move to the board
    /// Returns whether the move was legal or not
    #[must_use]
    pub fn makemove<const UPDATE_HASH: bool>(&mut self, mv: &Mv) -> bool {
        self.history.push(self.save_undo());

        let piece = self
            .get_side_piece_on(mv.from)
            .expect("No piece on move origin square");

        let captured = self.get_side_piece_on(mv.to);
        let old_castling = self.castling;

        self.halfmoves += 1;

        // Remove piece
        self.clear_square::<UPDATE_HASH>(mv.from);

        // Remove captured
        self.set_piece::<UPDATE_HASH>(piece, mv.to);

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
            self.clear_square::<UPDATE_HASH>(Square::from_file_rank(
                mv.to.get_x(),
                mv.to.get_y() - 1,
            ));
            self.halfmoves = 0;
        }
        if let Some(sq) = self.ep
            && piece == Piece::BP
            && mv.to == sq
        {
            self.clear_square::<UPDATE_HASH>(Square::from_file_rank(
                mv.to.get_x(),
                mv.to.get_y() + 1,
            ));
            self.halfmoves = 0;
        }

        // Clear EP square
        if UPDATE_HASH && let Some(sq) = self.ep {
            self.hash ^= ep_key(sq.get_x());
        }
        self.ep = None;

        // Set EP square?
        if piece == Piece::WP && mv.from.get_y() == 1 && mv.to.get_y() == 3 {
            self.ep = Some(Square::from_file_rank(mv.from.get_x(), 2));
        }
        if piece == Piece::BP && mv.from.get_y() == 6 && mv.to.get_y() == 4 {
            self.ep = Some(Square::from_file_rank(mv.from.get_x(), 5));
        }
        if UPDATE_HASH && let Some(sq) = self.ep {
            self.hash ^= ep_key(sq.get_x());
        }

        // Promotions
        match (self.turn, mv.promo) {
            (Side::White, Some(PromoPiece::Queen)) => {
                self.set_piece::<UPDATE_HASH>(Piece::WQ, mv.to)
            }
            (Side::White, Some(PromoPiece::Rook)) => {
                self.set_piece::<UPDATE_HASH>(Piece::WR, mv.to)
            }
            (Side::White, Some(PromoPiece::Bishop)) => {
                self.set_piece::<UPDATE_HASH>(Piece::WB, mv.to)
            }
            (Side::White, Some(PromoPiece::Knight)) => {
                self.set_piece::<UPDATE_HASH>(Piece::WN, mv.to)
            }
            (Side::Black, Some(PromoPiece::Queen)) => {
                self.set_piece::<UPDATE_HASH>(Piece::BQ, mv.to)
            }
            (Side::Black, Some(PromoPiece::Rook)) => {
                self.set_piece::<UPDATE_HASH>(Piece::BR, mv.to)
            }
            (Side::Black, Some(PromoPiece::Bishop)) => {
                self.set_piece::<UPDATE_HASH>(Piece::BB, mv.to)
            }
            (Side::Black, Some(PromoPiece::Knight)) => {
                self.set_piece::<UPDATE_HASH>(Piece::BN, mv.to)
            }
            (_, _) => {}
        }

        // Castling permissions - Did white's king rook move or get captured?
        if mv.from == Square::H1 || mv.to == Square::H1 {
            self.castling[Castling::WKS] = false;
        }

        // Castling permissions - Did white's queen rook move or get captured?
        if mv.from == Square::A1 || mv.to == Square::A1 {
            self.castling[Castling::WQS] = false;
        }

        // Castling permissions - Did white's king move?
        if mv.from == Square::E1 {
            self.castling[Castling::WKS] = false;
            self.castling[Castling::WQS] = false;
        }

        // Castling permissions - Did black's king rook move or get captured?
        if mv.from == Square::H8 || mv.to == Square::H8 {
            self.castling[Castling::BKS] = false;
        }

        // Castling permissions - Did black's queen rook move or get captured?
        if mv.from == Square::A8 || mv.to == Square::A8 {
            self.castling[Castling::BQS] = false;
        }

        // Castling permissions - Did black's king move?
        if mv.from == Square::E8 {
            self.castling[Castling::BKS] = false;
            self.castling[Castling::BQS] = false;
        }

        // Castling wks
        if piece == Piece::WK && mv.from == Square::E1 && mv.to == Square::G1 {
            self.clear_square::<UPDATE_HASH>(Square::H1);
            self.set_piece::<UPDATE_HASH>(Piece::WR, Square::F1);
        }

        // Castling wqs
        if piece == Piece::WK && mv.from == Square::E1 && mv.to == Square::C1 {
            self.clear_square::<UPDATE_HASH>(Square::A1);
            self.set_piece::<UPDATE_HASH>(Piece::WR, Square::D1);
        }

        // Castling bks
        if piece == Piece::BK && mv.from == Square::E8 && mv.to == Square::G8 {
            self.clear_square::<UPDATE_HASH>(Square::H8);
            self.set_piece::<UPDATE_HASH>(Piece::BR, Square::F8);
        }

        // Castling bqs
        if piece == Piece::BK && mv.from == Square::E8 && mv.to == Square::C8 {
            self.clear_square::<UPDATE_HASH>(Square::A8);
            self.set_piece::<UPDATE_HASH>(Piece::BR, Square::D8);
        }

        // Update the king square
        if piece == Piece::WK {
            self.ksq[Side::White] = Some(mv.to);
        }
        if piece == Piece::BK {
            self.ksq[Side::Black] = Some(mv.to);
        }

        // Fullmove counter increments after Black's move
        if self.turn == Side::Black {
            self.fullmoves += 1;
        }

        // Side to move
        self.turn = !self.turn;

        if UPDATE_HASH {
            self.hash ^= SIDE_KEY;

            // Castling rights can only be lost, never gained, during a move
            for right in Castling::ALL {
                if old_castling[right] && !self.castling[right] {
                    self.hash ^= castling_key(right);
                }
            }
        }

        // Legality check
        !self.is_attacked(self.ksq[!self.turn].unwrap(), self.turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makemove_legal() {
        let fen = "4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1";
        let legal = ["e3e2", "e3e7", "f5g7", "b7b8q", "b7b8n"];

        for movestr in legal {
            let mut pos = Position::from_fen(fen).unwrap();
            let mv = movestr.parse().unwrap();
            let success = pos.makemove::<true>(&mv);
            assert!(success, "Move \"{movestr}\" was meant to be legal");
        }
    }

    #[test]
    fn makemove_illegal() {
        let fen = "4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1";
        let illegal = ["e1d1", "e1d2", "e1e2", "e1f2", "e1f1", "e3d2", "e3b6"];

        for movestr in illegal {
            let mut pos = Position::from_fen(fen).unwrap();
            let mv = movestr.parse().unwrap();
            let success = pos.makemove::<true>(&mv);
            assert!(!success, "Move \"{movestr}\" was meant to be illegal");
        }
    }

    #[test]
    fn changes_white() {
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
            let mut pos = Position::from_fen(startfen).unwrap();
            let mv = movestr.parse().unwrap();
            let success = pos.makemove::<true>(&mv);

            assert!(success);
            assert_eq!(pos.get_fen(), fen);
        }
    }

    #[test]
    fn fullmoves_increment() {
        let mut pos = Position::from_fen("startpos").unwrap();

        assert!(pos.makemove::<true>(&"e2e4".parse().unwrap()));
        assert_eq!(
            pos.fullmoves, 1,
            "fullmoves shouldn't increment after White's move"
        );

        assert!(pos.makemove::<true>(&"e7e5".parse().unwrap()));
        assert_eq!(
            pos.fullmoves, 2,
            "fullmoves should increment after Black's move"
        );
    }
}
