use crate::square::Square;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    Length,
    FromSqOffboard,
    ToSqOffboard,
    IllegalPromo,
}

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
    pub fn from_string(movestr: &str) -> Result<Mv, ParseError> {
        if movestr.len() < 4 {
            Err(ParseError::Length)
        } else if movestr.len() > 5 {
            Err(ParseError::Length)
        } else {
            let from = if let Ok(sq) = Square::from_string(&movestr[0..2]) {
                sq
            } else {
                return Err(ParseError::FromSqOffboard);
            };

            let to = if let Ok(sq) = Square::from_string(&movestr[2..4]) {
                sq
            } else {
                return Err(ParseError::ToSqOffboard);
            };

            let promo = match movestr.chars().nth(4) {
                Some('n') => Some(PromoPiece::Knight),
                Some('b') => Some(PromoPiece::Bishop),
                Some('r') => Some(PromoPiece::Rook),
                Some('q') => Some(PromoPiece::Queen),
                Some(_) => return Err(ParseError::IllegalPromo),
                None => None,
            };

            Ok(Mv { from, to, promo })
        }
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

    static MOVESTRS_INVALID: [&str; 15] = [
        "", "a1", "a1a1a1", "0000", "A1a2", "a1A2", "a0a8", "a1a9", "a7a8p", "a7a8k", "a7a8N",
        "a7a8B", "a7a8R", "a7a8Q", "a7a8qq",
    ];

    #[test]
    fn test_parse_pass() {
        for movestr in MOVESTRS_VALID {
            assert!(Mv::from_string(movestr).is_ok());
        }
    }

    #[test]
    fn test_parse_fail() {
        for movestr in MOVESTRS_INVALID {
            assert!(Mv::from_string(movestr).is_err());
        }
    }

    #[test]
    fn test_to_string() {
        for movestr in MOVESTRS_VALID {
            assert_eq!(Mv::from_string(movestr).unwrap().to_string(), movestr);
        }
    }
}
