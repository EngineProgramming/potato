use crate::{Piece, Side, bitboard::Bitboard, position::Position, square::Square};

impl Position {
    /// Is the square attacked?
    #[must_use]
    pub fn is_attacked(&self, sq: Square, side: Side) -> bool {
        let bb = Bitboard::from_square(sq);
        let blockers = self.colours[Side::White as usize] | self.colours[Side::Black as usize];

        // Pawns
        if side == Side::White {
            if (bb.south().east()
                & self.pieces[Piece::Pawn as usize]
                & self.colours[Side::White as usize])
                .is_occupied()
            {
                return true;
            }
            if (bb.south().west()
                & self.pieces[Piece::Pawn as usize]
                & self.colours[Side::White as usize])
                .is_occupied()
            {
                return true;
            }
        } else {
            if (bb.north().east()
                & self.pieces[Piece::Pawn as usize]
                & self.colours[Side::Black as usize])
                .is_occupied()
            {
                return true;
            }
            if (bb.north().west()
                & self.pieces[Piece::Pawn as usize]
                & self.colours[Side::Black as usize])
                .is_occupied()
            {
                return true;
            }
        }

        // Knights
        if (Bitboard::mask_knight(sq, blockers)
            & self.pieces[Piece::Knight as usize]
            & self.colours[side as usize])
            .is_occupied()
        {
            return true;
        }

        // Bishops
        if (Bitboard::mask_bishop(sq, blockers)
            & self.pieces[Piece::Bishop as usize]
            & self.colours[side as usize])
            .is_occupied()
        {
            return true;
        }

        // Rooks
        if (Bitboard::mask_rook(sq, blockers)
            & self.pieces[Piece::Rook as usize]
            & self.colours[side as usize])
            .is_occupied()
        {
            return true;
        }

        // Queens
        if (Bitboard::mask_queen(sq, blockers)
            & self.pieces[Piece::Queen as usize]
            & self.colours[side as usize])
            .is_occupied()
        {
            return true;
        }

        // King
        if (Bitboard::mask_king(sq, blockers)
            & self.pieces[Piece::King as usize]
            & self.colours[side as usize])
            .is_occupied()
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_attacked_black() {
        let pos = Position::from_fen("4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1");

        // Attacked
        let attacked = ["d1", "e2", "f2", "f3", "g7", "f8"];
        for sqstr in attacked {
            let sq = Square::from_string(sqstr);
            assert!(pos.is_attacked(sq, Side::Black), "{}", sq);
        }

        // Not attacked
        let not_attacked = ["a2", "a4", "a8", "b8", "d5", "g1", "e1"];
        for sqstr in not_attacked {
            let sq = Square::from_string(sqstr);
            assert!(!pos.is_attacked(sq, Side::Black), "{}", sq);
        }
    }
}
