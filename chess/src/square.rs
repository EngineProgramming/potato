use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    /// Create a square from a string
    #[must_use]
    pub fn from_string(word: &str) -> Square {
        let f = word.chars().nth(0).expect("Failed to parse square file");
        let r = word.chars().nth(1).expect("Failed to parse square rank");

        if !('a'..='j').contains(&f) || !('1'..='8').contains(&r) {
            panic!("Out of range");
        }

        Self::from_file_rank(f as u8 - b'a', r as u8 - b'1')
    }

    /// Create a square from x and y coordinates
    #[must_use]
    pub fn from_file_rank(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Create a square from its index
    #[must_use]
    pub fn from_index(idx: u8) -> Self {
        Self {
            x: idx % 8,
            y: idx / 8,
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.x) as char, (b'1' + self.y) as char)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(Square::from_string("a1"), Square::from_file_rank(0, 0));
        assert_eq!(Square::from_string("a8"), Square::from_file_rank(0, 7));
        assert_eq!(Square::from_string("h1"), Square::from_file_rank(7, 0));
        assert_eq!(Square::from_string("h8"), Square::from_file_rank(7, 7));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Square::from_file_rank(0, 0).to_string(), "a1");
        assert_eq!(Square::from_file_rank(7, 0).to_string(), "h1");
        assert_eq!(Square::from_file_rank(0, 7).to_string(), "a8");
        assert_eq!(Square::from_file_rank(7, 7).to_string(), "h8");
    }
}
