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
}

impl Default for Position {
    fn default() -> Self {
        Self {
            board: [[None; 8]; 8],
            turn: Side::White,
            halfmoves: 0,
            fullmoves: 0,
            ep: None,
            castling: [false; 4],
        }
    }
}

impl Position {
    /// Return what piece, if any, is on the given square
    #[must_use]
    pub fn get_side_piece_on(&self, sq: Square) -> Option<Piece> {
        let x = sq.get_x() as usize;
        let y = sq.get_y() as usize;
        self.board[x][y]
    }

    /// Place a piece on the board
    pub fn set_piece(&mut self, piece: Piece, sq: Square) {
        let x = sq.get_x() as usize;
        let y = sq.get_y() as usize;
        self.board[x][y] = Some(piece);
    }
}
