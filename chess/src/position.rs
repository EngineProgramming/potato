use crate::{Piece, Side, bitboard::Bitboard, square::Square};

/// This struct holds all the information about a chess position.
#[derive(Clone, Copy)]
pub struct Position {
    pub colours: [Bitboard; 2],
    pub pieces: [Bitboard; 6],
    pub turn: Side,
    pub halfmoves: u8,
    pub fullmoves: u8,
    pub ep: Option<Square>,
    pub castling: [bool; 4],
}

impl Default for Position {
    fn default() -> Self {
        Self {
            colours: [Bitboard::from_empty(); 2],
            pieces: [Bitboard::from_empty(); 6],
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
    pub fn get_piece_on(&self, sq: Square) -> Option<Piece> {
        if self.pieces[Piece::Pawn as usize].is_set(sq) {
            Some(Piece::Pawn)
        } else if self.pieces[Piece::Knight as usize].is_set(sq) {
            Some(Piece::Knight)
        } else if self.pieces[Piece::Bishop as usize].is_set(sq) {
            Some(Piece::Bishop)
        } else if self.pieces[Piece::Rook as usize].is_set(sq) {
            Some(Piece::Rook)
        } else if self.pieces[Piece::Queen as usize].is_set(sq) {
            Some(Piece::Queen)
        } else if self.pieces[Piece::King as usize].is_set(sq) {
            Some(Piece::King)
        } else {
            None
        }
    }

    /// Return the colour of the piece, if any, that is on the given square
    #[must_use]
    pub fn get_colour_on(&self, sq: Square) -> Option<Side> {
        if self.colours[Side::White as usize].is_set(sq) {
            Some(Side::White)
        } else if self.colours[Side::Black as usize].is_set(sq) {
            Some(Side::Black)
        } else {
            None
        }
    }

    /// Is the given square empty?
    #[must_use]
    pub fn is_empty(&self, sq: Square) -> bool {
        !(self.colours[Side::White as usize] | self.colours[Side::Black as usize]).is_set(sq)
    }

    /// Place a piece on the board
    pub fn set_piece(&mut self, sq: Square, side: Side, piece: Piece) {
        self.colours[side as usize].set(sq);
        self.pieces[piece as usize].set(sq);
    }

    /// Clear a square on the board
    pub fn clear_square(&mut self, sq: Square) {
        self.colours[Side::White as usize].unset(sq);
        self.colours[Side::Black as usize].unset(sq);
        self.pieces[Piece::Pawn as usize].unset(sq);
        self.pieces[Piece::Knight as usize].unset(sq);
        self.pieces[Piece::Bishop as usize].unset(sq);
        self.pieces[Piece::Rook as usize].unset(sq);
        self.pieces[Piece::Queen as usize].unset(sq);
        self.pieces[Piece::King as usize].unset(sq);
    }
}
