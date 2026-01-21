use std::ops::Not;

pub mod fen;
pub mod position;
pub mod square;

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    WP,
    WN,
    WB,
    WR,
    WQ,
    WK,
    BP,
    BN,
    BB,
    BR,
    BQ,
    BK,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

pub enum Castling {
    WKS,
    WQS,
    BKS,
    BQS,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side() {
        assert_eq!(!Side::White, Side::Black);
        assert_eq!(!Side::Black, Side::White);
    }
}
