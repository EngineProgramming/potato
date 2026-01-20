use crate::square::Square;
use std::fmt::Display;
use std::str::FromStr;

/// Promotion types
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

impl FromStr for Mv {
    type Err = ();

    /// Parse a move from long algebraic notation (e.g. `"e2e4"`, `"a7a8q"`)
    fn from_str(movestr: &str) -> Result<Self, Self::Err> {
        let from = movestr.get(0..2).ok_or(())?.parse()?;
        let to = movestr.get(2..4).ok_or(())?.parse()?;
        let promo = match movestr.chars().nth(4) {
            Some('n') => Some(PromoPiece::Knight),
            Some('b') => Some(PromoPiece::Bishop),
            Some('r') => Some(PromoPiece::Rook),
            Some('q') => Some(PromoPiece::Queen),
            Some(_) => return Err(()),
            None => None,
        };
        Ok(Self { from, to, promo })
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
    fn parse() {
        for movestr in MOVESTRS_VALID {
            assert_eq!(movestr.parse::<Mv>().unwrap().to_string(), movestr);
        }
    }

    #[test]
    fn promo_piece_equality() {
        assert_eq!(PromoPiece::Queen, PromoPiece::Queen);
        assert_ne!(PromoPiece::Queen, PromoPiece::Rook);
        assert_ne!(PromoPiece::Bishop, PromoPiece::Knight);
    }

    #[test]
    fn parse_invalid() {
        for movestr in ["", "a1a", "i1a1", "a1i1", "a7a8x"] {
            assert!(movestr.parse::<Mv>().is_err(), "{movestr}");
        }
    }
}
