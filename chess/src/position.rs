use crate::{Piece, Side, square::Square};

/// This struct holds all the information about a chess position.
#[derive(Clone, Copy)]
pub struct Position {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Side,
    pub halfmoves: u8,
    pub fullmoves: u8,
    pub ep: Option<Square>,
    pub castling: [bool; 4],
    pub ksq: [Option<Square>; 2],
}
