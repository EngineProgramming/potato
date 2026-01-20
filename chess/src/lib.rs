use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Not;

pub mod position;
pub mod square;

#[derive(PartialEq, Eq, Clone, Copy)]
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

impl<T> Index<Piece> for [T; 12] {
    type Output = T;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece as usize]
    }
}

impl<T> IndexMut<Piece> for [T; 12] {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece as usize]
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

impl<T> Index<Side> for [T; 2] {
    type Output = T;

    fn index(&self, side: Side) -> &Self::Output {
        &self[side as usize]
    }
}

impl<T> IndexMut<Side> for [T; 2] {
    fn index_mut(&mut self, side: Side) -> &mut Self::Output {
        &mut self[side as usize]
    }
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side() {
        assert_eq!(!Side::White, Side::Black);
        assert_eq!(!Side::Black, Side::White);
    }
}
