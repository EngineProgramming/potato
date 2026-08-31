use crate::{Piece, Side, square::Square, undomove::Undo, zobrist::piece_key};

/// This struct holds all the information about a chess position.
#[derive(Clone)]
pub struct Position {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Side,
    pub halfmoves: u8,
    pub fullmoves: u8,
    pub ep: Option<Square>,
    pub castling: [bool; 4],
    pub ksq: [Option<Square>; 2],
    pub history: Vec<Undo>,
    pub hash: u64,
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
            ksq: [None; 2],
            history: Vec::new(),
            hash: 0,
        }
    }
}

impl Position {
    /// Return what piece, if any, is on the given square
    #[must_use]
    pub const fn get_side_piece_on(&self, sq: Square) -> Option<Piece> {
        self.board[sq.get_x() as usize][sq.get_y() as usize]
    }

    /// Place a piece on the board, optionally updating the hash incrementally
    pub(crate) fn set_piece<const UPDATE_HASH: bool>(&mut self, piece: Piece, sq: Square) {
        if UPDATE_HASH {
            if let Some(old) = self.get_side_piece_on(sq) {
                self.hash ^= piece_key(old, sq);
            }
            self.hash ^= piece_key(piece, sq);
        }
        self.board[sq.get_x() as usize][sq.get_y() as usize] = Some(piece);
    }

    /// Clear a square on the board, optionally updating the hash incrementally
    pub(crate) fn clear_square<const UPDATE_HASH: bool>(&mut self, sq: Square) {
        if UPDATE_HASH && let Some(old) = self.get_side_piece_on(sq) {
            self.hash ^= piece_key(old, sq);
        }
        self.board[sq.get_x() as usize][sq.get_y() as usize] = None;
    }

    /// Get the colour of the piece on a given square
    #[must_use]
    pub const fn get_side_on(&self, sq: Square) -> Option<Side> {
        match self.get_side_piece_on(sq) {
            Some(Piece::WP | Piece::WN | Piece::WB | Piece::WR | Piece::WQ | Piece::WK) => {
                Some(Side::White)
            }
            Some(Piece::BP | Piece::BN | Piece::BB | Piece::BR | Piece::BQ | Piece::BK) => {
                Some(Side::Black)
            }
            None => None,
        }
    }
}
