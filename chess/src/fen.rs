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
                    self.set_piece(Piece::WP, sq);
                    x += 1;
                }
                'N' => {
                    self.set_piece(Piece::WN, sq);
                    x += 1;
                }
                'B' => {
                    self.set_piece(Piece::WB, sq);
                    x += 1;
                }
                'R' => {
                    self.set_piece(Piece::WR, sq);
                    x += 1;
                }
                'Q' => {
                    self.set_piece(Piece::WQ, sq);
                    x += 1;
                }
                'K' => {
                    self.set_piece(Piece::WK, sq);
                    self.ksq[Side::White as usize] = Some(sq);
                    x += 1;
                }
                'p' => {
                    self.set_piece(Piece::BP, sq);
                    x += 1;
                }
                'n' => {
                    self.set_piece(Piece::BN, sq);
                    x += 1;
                }
                'b' => {
                    self.set_piece(Piece::BB, sq);
                    x += 1;
                }
                'r' => {
                    self.set_piece(Piece::BR, sq);
                    x += 1;
                }
                'q' => {
                    self.set_piece(Piece::BQ, sq);
                    x += 1;
                }
                'k' => {
                    self.set_piece(Piece::BK, sq);
                    self.ksq[Side::Black as usize] = Some(sq);
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
                let found = self.get_side_piece_on(sq);

                if found.is_some() && empty > 0 {
                    fen += &empty.to_string();
                    empty = 0;
                }

                match found {
                    Some(Piece::WP) => fen += "P",
                    Some(Piece::WN) => fen += "N",
                    Some(Piece::WB) => fen += "B",
                    Some(Piece::WR) => fen += "R",
                    Some(Piece::WQ) => fen += "Q",
                    Some(Piece::WK) => fen += "K",
                    Some(Piece::BP) => fen += "p",
                    Some(Piece::BN) => fen += "n",
                    Some(Piece::BB) => fen += "b",
                    Some(Piece::BR) => fen += "r",
                    Some(Piece::BQ) => fen += "q",
                    Some(Piece::BK) => fen += "k",
                    None => empty += 1,
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
