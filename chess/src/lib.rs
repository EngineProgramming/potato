use std::ops::Not;

pub mod bitboard;
pub mod fen;
pub mod is_attacked;
pub mod makemove;
pub mod movegen;
pub mod mv;
pub mod perft;
pub mod position;
pub mod square;

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
