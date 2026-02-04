use crate::{Castling, Piece, Side, position::Position, square::Square};

impl Position {
    /// Create a new position from the FEN given
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let mut pos = Self::default();
        pos.set_fen(fen);
        pos
    }

    /// Set the position to the FEN given
    pub fn set_fen(&mut self, fen: &str) {
        if fen == "startpos" {
            return self.set_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }

        *self = Position::default();

        let parts = fen.split(" ").collect::<Vec<&str>>();

        // Pieces
        let mut x = 0i32;
        let mut y = 7i32;
        for c in parts[0].chars() {
            let sq = Square::from_file_rank(x as u8, y as u8);

            match c {
                'P' => {
                    self.set_piece(sq, Side::White, Piece::Pawn);
                    x += 1;
                }
                'N' => {
                    self.set_piece(sq, Side::White, Piece::Knight);
                    x += 1;
                }
                'B' => {
                    self.set_piece(sq, Side::White, Piece::Bishop);
                    x += 1;
                }
                'R' => {
                    self.set_piece(sq, Side::White, Piece::Rook);
                    x += 1;
                }
                'Q' => {
                    self.set_piece(sq, Side::White, Piece::Queen);
                    x += 1;
                }
                'K' => {
                    self.set_piece(sq, Side::White, Piece::King);
                    x += 1;
                }
                'p' => {
                    self.set_piece(sq, Side::Black, Piece::Pawn);
                    x += 1;
                }
                'n' => {
                    self.set_piece(sq, Side::Black, Piece::Knight);
                    x += 1;
                }
                'b' => {
                    self.set_piece(sq, Side::Black, Piece::Bishop);
                    x += 1;
                }
                'r' => {
                    self.set_piece(sq, Side::Black, Piece::Rook);
                    x += 1;
                }
                'q' => {
                    self.set_piece(sq, Side::Black, Piece::Queen);
                    x += 1;
                }
                'k' => {
                    self.set_piece(sq, Side::Black, Piece::King);
                    x += 1;
                }
                '1'..='8' => {
                    x += (c as u8 - b'0') as i32;
                }
                '/' => {
                    x = 0;
                    y -= 1;
                }
                _ => panic!("Unrecognised piece"),
            }
        }

        // Side to move
        match parts[1] {
            "w" => self.turn = Side::White,
            "b" => self.turn = Side::Black,
            _ => panic!("Unrecognised side to move"),
        }

        // Castling permissions
        if parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => self.castling[Castling::WKS as usize] = true,
                    'Q' => self.castling[Castling::WQS as usize] = true,
                    'k' => self.castling[Castling::BKS as usize] = true,
                    'q' => self.castling[Castling::BQS as usize] = true,
                    _ => panic!("Invalid castling permission"),
                }
            }
        }

        // En Passant
        if parts[3] == "-" {
            self.ep = None;
        } else {
            self.ep = Some(Square::from_string(parts[3]));
        }

        // Halfmoves
        self.halfmoves = parts[4].parse::<u8>().unwrap();

        // Fullmoves
        self.fullmoves = parts[5].parse::<u8>().unwrap();
    }

    /// Get the FEN of the position
    #[must_use]
    pub fn get_fen(&self) -> String {
        let mut fen = String::new();

        // Pieces
        for y in (0..=7).rev() {
            let mut empty = 0;

            for x in 0..=7 {
                let sq = Square::from_file_rank(x, y);
                let piece = self.get_piece_on(sq);
                let colour = self.get_colour_on(sq);

                if piece.is_some() && empty > 0 {
                    fen += &empty.to_string();
                    empty = 0;
                }

                match (colour, piece) {
                    (Some(Side::White), Some(Piece::Pawn)) => fen += "P",
                    (Some(Side::White), Some(Piece::Knight)) => fen += "N",
                    (Some(Side::White), Some(Piece::Bishop)) => fen += "B",
                    (Some(Side::White), Some(Piece::Rook)) => fen += "R",
                    (Some(Side::White), Some(Piece::Queen)) => fen += "Q",
                    (Some(Side::White), Some(Piece::King)) => fen += "K",
                    (Some(Side::Black), Some(Piece::Pawn)) => fen += "p",
                    (Some(Side::Black), Some(Piece::Knight)) => fen += "n",
                    (Some(Side::Black), Some(Piece::Bishop)) => fen += "b",
                    (Some(Side::Black), Some(Piece::Rook)) => fen += "r",
                    (Some(Side::Black), Some(Piece::Queen)) => fen += "q",
                    (Some(Side::Black), Some(Piece::King)) => fen += "k",
                    (_, _) => empty += 1,
                }
            }

            if empty > 0 {
                fen += &empty.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        // Side to move
        match self.turn {
            Side::White => fen += " w",
            Side::Black => fen += " b",
        }

        // Castling
        fen += " ";
        if self.castling[Castling::WKS as usize] {
            fen += "K";
        }
        if self.castling[Castling::WQS as usize] {
            fen += "Q";
        }
        if self.castling[Castling::BKS as usize] {
            fen += "k";
        }
        if self.castling[Castling::BQS as usize] {
            fen += "q";
        }
        if !self.castling[Castling::WKS as usize]
            && !self.castling[Castling::WQS as usize]
            && !self.castling[Castling::BKS as usize]
            && !self.castling[Castling::BQS as usize]
        {
            fen += "-";
        }

        // En passant
        if let Some(sq) = &self.ep {
            fen += " ";
            fen += &sq.to_string();
        } else {
            fen += " -";
        }

        // Halfmoves
        fen += " ";
        fen += &self.halfmoves.to_string();

        // Fullmoves
        fen += " ";
        fen += &self.fullmoves.to_string();

        fen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static FENS_VALID: [&str; 11] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b kq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1",
        "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
        "rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w KQkq a6 0 2",
        "rnbqkbnr/1ppppppp/8/p7/P6P/8/1PPPPPP1/RNBQKBNR b KQkq h3 0 2",
        "rnbqkbnr/1pppppp1/8/p6p/P6P/8/1PPPPPP1/RNBQKBNR w KQkq h6 0 3",
        "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6",
    ];

    #[test]
    fn test_startpos() {
        let pos = Position::from_fen("startpos");
        assert_eq!(
            pos.get_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_valid() {
        for fen in FENS_VALID {
            let pos = Position::from_fen(fen);
            assert_eq!(pos.get_fen(), fen);
        }
    }

    #[test]
    fn test_fen_override() {
        let mut pos = Position::default();
        for fen in FENS_VALID {
            pos.set_fen(fen);
            assert_eq!(pos.get_fen(), fen, "set_fen() override failure");
        }
    }
}
