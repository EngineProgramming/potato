use crate::{Castling, Piece, Side, position::Position, square::Square};

impl Position {
    /// Create a new position from the FEN given
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[allow(clippy::result_unit_err)]
    pub fn from_fen(fen: &str) -> Result<Self, ()> {
        if fen == "startpos" {
            return Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }

        let mut pos = Self::default();

        let parts = fen.split(' ').collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(());
        }

        // Pieces
        let mut x = 0i32;
        let mut y = 7i32;
        for c in parts[0].chars() {
            let sq = Square::from_file_rank(x as u8, y as u8);

            match c {
                'P' => {
                    pos.set_piece::<false>(Piece::WP, sq);
                    x += 1;
                }
                'N' => {
                    pos.set_piece::<false>(Piece::WN, sq);
                    x += 1;
                }
                'B' => {
                    pos.set_piece::<false>(Piece::WB, sq);
                    x += 1;
                }
                'R' => {
                    pos.set_piece::<false>(Piece::WR, sq);
                    x += 1;
                }
                'Q' => {
                    pos.set_piece::<false>(Piece::WQ, sq);
                    x += 1;
                }
                'K' => {
                    pos.set_piece::<false>(Piece::WK, sq);
                    pos.ksq[Side::White] = Some(sq);
                    x += 1;
                }
                'p' => {
                    pos.set_piece::<false>(Piece::BP, sq);
                    x += 1;
                }
                'n' => {
                    pos.set_piece::<false>(Piece::BN, sq);
                    x += 1;
                }
                'b' => {
                    pos.set_piece::<false>(Piece::BB, sq);
                    x += 1;
                }
                'r' => {
                    pos.set_piece::<false>(Piece::BR, sq);
                    x += 1;
                }
                'q' => {
                    pos.set_piece::<false>(Piece::BQ, sq);
                    x += 1;
                }
                'k' => {
                    pos.set_piece::<false>(Piece::BK, sq);
                    pos.ksq[Side::Black] = Some(sq);
                    x += 1;
                }
                '1'..='8' => {
                    x += i32::from(c as u8 - b'0');
                }
                '/' => {
                    x = 0;
                    y -= 1;
                }
                _ => return Err(()),
            }
        }

        // Side to move
        match parts[1] {
            "w" => pos.turn = Side::White,
            "b" => pos.turn = Side::Black,
            _ => return Err(()),
        }

        // Castling permissions
        if parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => pos.castling[Castling::WKS] = true,
                    'Q' => pos.castling[Castling::WQS] = true,
                    'k' => pos.castling[Castling::BKS] = true,
                    'q' => pos.castling[Castling::BQS] = true,
                    _ => return Err(()),
                }
            }
        }

        // En Passant
        if parts[3] == "-" {
            pos.ep = None;
        } else {
            pos.ep = Some(parts[3].parse()?);
        }

        // Halfmoves
        pos.halfmoves = parts[4].parse().map_err(|_| ())?;

        // Fullmoves
        pos.fullmoves = parts[5].parse().map_err(|_| ())?;

        // Both sides must have a king - the rest of the engine assumes this
        if pos.ksq[Side::White].is_none() || pos.ksq[Side::Black].is_none() {
            return Err(());
        }

        pos.hash = pos.calculate_hash();

        Ok(pos)
    }

    /// Set the position to the FEN given
    #[allow(clippy::result_unit_err)]
    pub fn set_fen(&mut self, fen: &str) -> Result<(), ()> {
        *self = Self::from_fen(fen)?;
        Ok(())
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
        if self.castling[Castling::WKS] {
            fen += "K";
        }
        if self.castling[Castling::WQS] {
            fen += "Q";
        }
        if self.castling[Castling::BKS] {
            fen += "k";
        }
        if self.castling[Castling::BQS] {
            fen += "q";
        }
        if !self.castling[Castling::WKS]
            && !self.castling[Castling::WQS]
            && !self.castling[Castling::BKS]
            && !self.castling[Castling::BQS]
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
    fn startpos() {
        let pos = Position::from_fen("startpos").unwrap();
        assert_eq!(
            pos.get_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn valid() {
        for fen in FENS_VALID {
            let pos = Position::from_fen(fen).unwrap();
            assert_eq!(pos.get_fen(), fen);
        }
    }

    #[test]
    fn fen_override() {
        let mut pos = Position::default();
        for fen in FENS_VALID {
            pos.set_fen(fen).unwrap();
            assert_eq!(pos.get_fen(), fen, "set_fen() override failure");
        }
    }

    #[test]
    fn invalid() {
        let cases = [
            "",
            "8/8/8/8/8/8/8/8 w KQkq - 0",
            "8/8/8/8/8/8/8/X7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/8 x KQkq - 0 1",
            "8/8/8/8/8/8/8/8 w XQkq - 0 1",
            "8/8/8/8/8/8/8/8 w KQkq x9 0 1",
            "8/8/8/8/8/8/8/8 w KQkq - x 1",
            "8/8/8/8/8/8/8/8 w KQkq - 0 x",
            "8/8/8/8/8/8/8/8 w - - 0 1",
            "4k3/8/8/8/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/8/8/4K3 w - - 0 1",
        ];

        for fen in cases {
            assert!(Position::from_fen(fen).is_err(), "{fen}");
        }
    }

    #[test]
    fn set_fen_unchanged_on_error() {
        let mut pos = Position::from_fen("startpos").unwrap();
        let before = pos.get_fen();

        assert!(pos.set_fen("not a valid fen").is_err());

        assert_eq!(pos.get_fen(), before);
    }
}
