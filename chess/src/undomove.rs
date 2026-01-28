use crate::{Piece, Side, position::Position, square::Square};

/// Snapshot of the position state needed to undo a move
#[derive(Clone, Copy)]
pub struct Undo {
    board: [[Option<Piece>; 8]; 8],
    turn: Side,
    halfmoves: u8,
    fullmoves: u8,
    ep: Option<Square>,
    castling: [bool; 4],
    ksq: [Option<Square>; 2],
}

impl Position {
    pub(crate) fn save_undo(&self) -> Undo {
        Undo {
            board: self.board,
            turn: self.turn,
            halfmoves: self.halfmoves,
            fullmoves: self.fullmoves,
            ep: self.ep,
            castling: self.castling,
            ksq: self.ksq,
        }
    }

    /// Undo the last move made with `makemove`
    pub fn undomove(&mut self) {
        let undo = self.history.pop().expect("No move to undo");
        self.board = undo.board;
        self.turn = undo.turn;
        self.halfmoves = undo.halfmoves;
        self.fullmoves = undo.fullmoves;
        self.ep = undo.ep;
        self.castling = undo.castling;
        self.ksq = undo.ksq;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undomove_restores_position() {
        let fen = "r3k2r/6P1/8/3pP3/8/8/4P3/R3K2R w KQkq d6 0 1";
        let moves = [
            "e5e6", "e2e3", "e2e4", "e5d6", "g7g8q", "e1g1", "e1c1", "h1h8", "a1a8",
        ];

        for movestr in moves {
            let mut pos = Position::from_fen(fen).unwrap();
            let before = pos.get_fen();
            let mv = movestr.parse().unwrap();

            let _ = pos.makemove(&mv);
            pos.undomove();

            assert_eq!(pos.get_fen(), before, "{movestr}");
        }
    }
}
