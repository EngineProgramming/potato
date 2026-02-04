use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::square::Square;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[must_use]
    pub const fn from_empty() -> Self {
        Self(0x0)
    }

    #[must_use]
    pub const fn from_full() -> Self {
        Self(0xFFFFFFFFFFFFFFFF)
    }

    #[must_use]
    pub const fn from_square(sq: Square) -> Self {
        Self(0x1 << sq.get_index())
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.0 == 0xFFFFFFFFFFFFFFFF
    }

    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn pop_lsb(&mut self) -> Square {
        let index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Square {
            x: index % 8,
            y: index / 8,
        }
    }

    #[must_use]
    pub const fn is_set(&self, sq: Square) -> bool {
        (self.0 >> sq.get_index()) & 1 == 1
    }

    pub fn set(&mut self, sq: Square) {
        self.0 |= 1u64 << sq.get_index();
    }

    pub fn unset(&mut self, sq: Square) {
        self.0 &= !(1u64 << sq.get_index());
    }

    #[must_use]
    pub const fn north(&self) -> Self {
        Self(self.0 << 8)
    }

    #[must_use]
    pub const fn south(&self) -> Self {
        Self(self.0 >> 8)
    }

    #[must_use]
    pub const fn east(&self) -> Self {
        Self((self.0 << 1) & 0xfefefefefefefefe)
    }

    #[must_use]
    pub const fn west(&self) -> Self {
        Self((self.0 >> 1) & 0x7f7f7f7f7f7f7f7f)
    }

    #[must_use]
    pub const fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    #[must_use]
    pub fn mask_pawn(sq: Square) -> Self {
        let mut bb = Bitboard::from_empty();
        bb |= Self::from_square(sq).north().east();
        bb |= Self::from_square(sq).north().west();
        bb
    }

    #[must_use]
    pub fn mask_knight(sq: Square, _: Self) -> Self {
        let mut bb = Bitboard::from_empty();
        bb |= Self::from_square(sq).north().north().east();
        bb |= Self::from_square(sq).north().north().west();
        bb |= Self::from_square(sq).south().south().east();
        bb |= Self::from_square(sq).south().south().west();
        bb |= Self::from_square(sq).east().east().north();
        bb |= Self::from_square(sq).east().east().south();
        bb |= Self::from_square(sq).west().west().north();
        bb |= Self::from_square(sq).west().west().south();
        bb
    }

    #[must_use]
    pub fn mask_bishop(sq: Square, blockers: Self) -> Self {
        let bb = Self::from_square(sq);

        let mut ne = bb.north().east();
        ne |= (ne & !blockers).north().east();
        ne |= (ne & !blockers).north().east();
        ne |= (ne & !blockers).north().east();
        ne |= (ne & !blockers).north().east();
        ne |= (ne & !blockers).north().east();
        ne |= (ne & !blockers).north().east();

        let mut nw = bb.north().west();
        nw |= (nw & !blockers).north().west();
        nw |= (nw & !blockers).north().west();
        nw |= (nw & !blockers).north().west();
        nw |= (nw & !blockers).north().west();
        nw |= (nw & !blockers).north().west();
        nw |= (nw & !blockers).north().west();

        let mut se = bb.south().east();
        se |= (se & !blockers).south().east();
        se |= (se & !blockers).south().east();
        se |= (se & !blockers).south().east();
        se |= (se & !blockers).south().east();
        se |= (se & !blockers).south().east();
        se |= (se & !blockers).south().east();

        let mut sw = bb.south().west();
        sw |= (sw & !blockers).south().west();
        sw |= (sw & !blockers).south().west();
        sw |= (sw & !blockers).south().west();
        sw |= (sw & !blockers).south().west();
        sw |= (sw & !blockers).south().west();
        sw |= (sw & !blockers).south().west();

        ne | nw | se | sw
    }

    #[must_use]
    pub fn mask_rook(sq: Square, blockers: Self) -> Self {
        let bb = Self::from_square(sq);

        let mut north = bb.north();
        north |= (north & !blockers).north();
        north |= (north & !blockers).north();
        north |= (north & !blockers).north();
        north |= (north & !blockers).north();
        north |= (north & !blockers).north();
        north |= (north & !blockers).north();

        let mut east = bb.east();
        east |= (east & !blockers).east();
        east |= (east & !blockers).east();
        east |= (east & !blockers).east();
        east |= (east & !blockers).east();
        east |= (east & !blockers).east();
        east |= (east & !blockers).east();

        let mut south = bb.south();
        south |= (south & !blockers).south();
        south |= (south & !blockers).south();
        south |= (south & !blockers).south();
        south |= (south & !blockers).south();
        south |= (south & !blockers).south();
        south |= (south & !blockers).south();

        let mut west = bb.west();
        west |= (west & !blockers).west();
        west |= (west & !blockers).west();
        west |= (west & !blockers).west();
        west |= (west & !blockers).west();
        west |= (west & !blockers).west();
        west |= (west & !blockers).west();

        north | east | south | west
    }

    #[must_use]
    pub fn mask_queen(sq: Square, blockers: Self) -> Self {
        Self::mask_bishop(sq, blockers) | Self::mask_rook(sq, blockers)
    }

    #[must_use]
    pub fn mask_king(sq: Square, _: Bitboard) -> Self {
        let bb = Bitboard::from_square(sq);
        Self(
            // North
            (bb.0 << 8) |
            // South
            (bb.0 >> 8) |
            // north west, south west, west
            (((bb.0 << 7) | (bb.0 >> 9) | (bb.0 >> 1)) & 0x7f7f7f7f7f7f7f7f) |
            // north east, south east, east
            (((bb.0 >> 7) | (bb.0 << 9) | (bb.0 << 1)) & 0xfefefefefefefefe),
        )
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..8).rev() {
            for x in 0..8 {
                let idx = 8 * y + x;
                if (self.0 >> idx) & 1 == 1 {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{bitboard::Bitboard, square::Square};

    #[test]
    fn count_tests() {
        assert!(Bitboard(0).count() == 0);
        assert!(Bitboard(1).count() == 1);
        assert!(Bitboard(2).count() == 1);
        assert!(Bitboard(3).count() == 2);
        assert!(Bitboard::from_empty().count() == 0);
        assert!(Bitboard::from_full().count() == 64);
    }

    #[test]
    fn empty() {
        assert!(Bitboard(0).is_empty());
        assert!(!Bitboard(1).is_empty());
    }

    #[test]
    fn bitxor() {
        assert!(Bitboard(0) ^ Bitboard(0) == Bitboard(0));
        assert!(Bitboard(1) ^ Bitboard(2) == Bitboard(3));
    }

    #[test]
    fn bitor() {
        assert!(Bitboard(1) | Bitboard(2) == Bitboard(3));
    }

    #[test]
    fn bitand() {
        assert!(Bitboard(1) & Bitboard(2) == Bitboard(0));
    }

    #[test]
    fn bitnot() {
        assert_eq!(!Bitboard(0), Bitboard(0xFFFFFFFFFFFFFFFF));
        assert_eq!(!Bitboard(0xFFFFFFFFFFFFFFFF), Bitboard(0));
    }

    #[test]
    fn north() {
        assert_eq!(Bitboard(0x0).north(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).north(), Bitboard(0x100));
        assert_eq!(Bitboard(0xff).north(), Bitboard(0xff00));
        assert_eq!(Bitboard(0xff0000000000).north(), Bitboard(0xff000000000000));
        assert_eq!(Bitboard(0xff00000000000000).north(), Bitboard(0x0));
    }

    #[test]
    fn south() {
        assert_eq!(Bitboard(0x0).south(), Bitboard(0x0));
        assert_eq!(Bitboard(0x100).south(), Bitboard(0x1));
        assert_eq!(Bitboard(0xff00).south(), Bitboard(0xff));
        assert_eq!(Bitboard(0xff000000000000).south(), Bitboard(0xff0000000000));
    }

    #[test]
    fn east() {
        assert_eq!(Bitboard(0x0).east(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).east(), Bitboard(0x2));
    }

    #[test]
    fn west() {
        assert_eq!(Bitboard(0x0).west(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).west(), Bitboard(0x0));
    }

    #[test]
    fn test_mask_bishop() {
        let tests = [
            (
                Square::from_index(0),
                Bitboard(0x0),
                Bitboard(0x8040201008040200),
            ),
            (
                Square::from_index(63),
                Bitboard(0x0),
                Bitboard(0x40201008040201),
            ),
            (
                Square::from_index(56),
                Bitboard(0x0),
                Bitboard(0x2040810204080),
            ),
            (
                Square::from_index(7),
                Bitboard(0x0),
                Bitboard(0x102040810204000),
            ),
            (
                Square::from_index(28),
                Bitboard(0x0),
                Bitboard(0x182442800284482),
            ),
            (
                Square::from_index(28),
                Bitboard(0x100400000200400),
                Bitboard(0x102442800280400),
            ),
        ];

        for (sq, blockers, mask) in tests {
            assert_eq!(Bitboard::mask_bishop(sq, blockers), mask);
        }
    }

    #[test]
    fn test_mask_rook() {
        let tests = [
            (
                Square::from_index(0),
                Bitboard(0x0),
                Bitboard(0x1010101010101fe),
            ),
            (
                Square::from_index(63),
                Bitboard(0x0),
                Bitboard(0x7f80808080808080),
            ),
            (
                Square::from_index(28),
                Bitboard(0x0),
                Bitboard(0x10101010ef101010),
            ),
            (
                Square::from_index(28),
                Bitboard(0x1000000008001000),
                Bitboard(0x10101010e8101000),
            ),
        ];

        for (sq, blockers, mask) in tests {
            assert_eq!(Bitboard::mask_rook(sq, blockers), mask);
        }
    }
}
