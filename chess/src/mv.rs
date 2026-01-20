use crate::square::Square;
use std::fmt::Display;

/// Promotion types
#[derive(Copy, Clone)]
pub enum PromoPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

/// The move struct
pub struct Mv {
    pub from: Square,
    pub to: Square,
    pub promo: Option<PromoPiece>,
}

impl Mv {
    /// Convert a string to a move
    #[must_use]
    pub fn from_string(movestr: &str) -> Mv {
        let from = Square::from_string(&movestr[0..2]);
        let to = Square::from_string(&movestr[2..4]);
        let promo = match movestr.chars().nth(4) {
            Some('n') => Some(PromoPiece::Knight),
            Some('b') => Some(PromoPiece::Bishop),
            Some('r') => Some(PromoPiece::Rook),
            Some('q') => Some(PromoPiece::Queen),
            Some(_) => panic!("Invalid promotion character"),
            None => None,
        };
        Mv { from, to, promo }
    }
}

impl Display for Mv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(piece) = &self.promo {
            write!(
                f,
                "{}{}{}",
                self.from,
                self.to,
                match piece {
                    PromoPiece::Knight => "n",
                    PromoPiece::Bishop => "b",
                    PromoPiece::Rook => "r",
                    PromoPiece::Queen => "q",
                }
            )?;
        } else {
            write!(f, "{}{}", self.from, self.to)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOVESTRS_VALID: [&str; 12] = [
        "a1a8", "h1h8", "a1h1", "a8h8", "a7a8n", "a7a8b", "a7a8r", "a7a8q", "a2a1n", "a2a1b",
        "a2a1r", "a2a1q",
    ];

    #[test]
    fn test_parse() {
        for movestr in MOVESTRS_VALID {
            assert_eq!(Mv::from_string(movestr).to_string(), movestr);
        }
    }
}
