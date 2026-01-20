use std::fmt::Display;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Square(u8);

impl FromStr for Square {
    type Err = ();

    /// Parse a square from algebraic notation (e.g. `"e4"`)
    fn from_str(word: &str) -> Result<Self, Self::Err> {
        let mut chars = word.chars();
        let f = chars.next().ok_or(())?;
        let r = chars.next().ok_or(())?;

        if !('a'..='h').contains(&f) || !('1'..='8').contains(&r) {
            return Err(());
        }

        Ok(Self::from_file_rank(f as u8 - b'a', r as u8 - b'1'))
    }
}

impl Square {
    pub const A1: Self = Self::from_index(0);
    pub const B1: Self = Self::from_index(1);
    pub const C1: Self = Self::from_index(2);
    pub const D1: Self = Self::from_index(3);
    pub const E1: Self = Self::from_index(4);
    pub const F1: Self = Self::from_index(5);
    pub const G1: Self = Self::from_index(6);
    pub const H1: Self = Self::from_index(7);

    pub const A2: Self = Self::from_index(8);
    pub const B2: Self = Self::from_index(9);
    pub const C2: Self = Self::from_index(10);
    pub const D2: Self = Self::from_index(11);
    pub const E2: Self = Self::from_index(12);
    pub const F2: Self = Self::from_index(13);
    pub const G2: Self = Self::from_index(14);
    pub const H2: Self = Self::from_index(15);

    pub const A3: Self = Self::from_index(16);
    pub const B3: Self = Self::from_index(17);
    pub const C3: Self = Self::from_index(18);
    pub const D3: Self = Self::from_index(19);
    pub const E3: Self = Self::from_index(20);
    pub const F3: Self = Self::from_index(21);
    pub const G3: Self = Self::from_index(22);
    pub const H3: Self = Self::from_index(23);

    pub const A4: Self = Self::from_index(24);
    pub const B4: Self = Self::from_index(25);
    pub const C4: Self = Self::from_index(26);
    pub const D4: Self = Self::from_index(27);
    pub const E4: Self = Self::from_index(28);
    pub const F4: Self = Self::from_index(29);
    pub const G4: Self = Self::from_index(30);
    pub const H4: Self = Self::from_index(31);

    pub const A5: Self = Self::from_index(32);
    pub const B5: Self = Self::from_index(33);
    pub const C5: Self = Self::from_index(34);
    pub const D5: Self = Self::from_index(35);
    pub const E5: Self = Self::from_index(36);
    pub const F5: Self = Self::from_index(37);
    pub const G5: Self = Self::from_index(38);
    pub const H5: Self = Self::from_index(39);

    pub const A6: Self = Self::from_index(40);
    pub const B6: Self = Self::from_index(41);
    pub const C6: Self = Self::from_index(42);
    pub const D6: Self = Self::from_index(43);
    pub const E6: Self = Self::from_index(44);
    pub const F6: Self = Self::from_index(45);
    pub const G6: Self = Self::from_index(46);
    pub const H6: Self = Self::from_index(47);

    pub const A7: Self = Self::from_index(48);
    pub const B7: Self = Self::from_index(49);
    pub const C7: Self = Self::from_index(50);
    pub const D7: Self = Self::from_index(51);
    pub const E7: Self = Self::from_index(52);
    pub const F7: Self = Self::from_index(53);
    pub const G7: Self = Self::from_index(54);
    pub const H7: Self = Self::from_index(55);

    pub const A8: Self = Self::from_index(56);
    pub const B8: Self = Self::from_index(57);
    pub const C8: Self = Self::from_index(58);
    pub const D8: Self = Self::from_index(59);
    pub const E8: Self = Self::from_index(60);
    pub const F8: Self = Self::from_index(61);
    pub const G8: Self = Self::from_index(62);
    pub const H8: Self = Self::from_index(63);

    /// Create a square from x and y coordinates
    #[must_use]
    pub const fn from_file_rank(x: u8, y: u8) -> Self {
        Self::from_index(y * 8 + x)
    }

    /// Create a square from its index
    #[must_use]
    pub const fn from_index(idx: u8) -> Self {
        Self(idx)
    }

    /// Get index
    #[must_use]
    pub const fn get_index(&self) -> u8 {
        self.0
    }

    /// Get x coordinate
    #[must_use]
    pub const fn get_x(&self) -> u8 {
        self.0 % 8
    }

    /// Get y coordinate
    #[must_use]
    pub const fn get_y(&self) -> u8 {
        self.0 / 8
    }
}

impl<T> Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, sq: Square) -> &Self::Output {
        &self[sq.0 as usize]
    }
}

impl<T> IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        &mut self[sq.0 as usize]
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + self.get_x()) as char,
            (b'1' + self.get_y()) as char
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("a1".parse(), Ok(Square::from_file_rank(0, 0)));
        assert_eq!("a8".parse(), Ok(Square::from_file_rank(0, 7)));
        assert_eq!("h1".parse(), Ok(Square::from_file_rank(7, 0)));
        assert_eq!("h8".parse(), Ok(Square::from_file_rank(7, 7)));
    }

    #[test]
    fn from_str_invalid() {
        for word in ["", "a", "i1", "a9", "11", "aa"] {
            assert!(word.parse::<Square>().is_err(), "{word}");
        }
    }

    #[test]
    fn constants() {
        assert_eq!(Square::A1, Square::from_file_rank(0, 0));
        assert_eq!(Square::H1, Square::from_file_rank(7, 0));
        assert_eq!(Square::A8, Square::from_file_rank(0, 7));
        assert_eq!(Square::H8, Square::from_file_rank(7, 7));
        assert_eq!(Square::E4, Square::from_file_rank(4, 3));
    }

    #[test]
    fn index_array() {
        let mut arr = [0u8; 64];
        arr[Square::E4] = 42;
        assert_eq!(arr[Square::E4], 42);
        assert_eq!(arr[Square::A1], 0);
    }

    #[test]
    fn to_string() {
        assert_eq!(Square::from_file_rank(0, 0).to_string(), "a1");
        assert_eq!(Square::from_file_rank(7, 0).to_string(), "h1");
        assert_eq!(Square::from_file_rank(0, 7).to_string(), "a8");
        assert_eq!(Square::from_file_rank(7, 7).to_string(), "h8");
    }
}
