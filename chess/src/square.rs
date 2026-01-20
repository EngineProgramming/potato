use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    IllegalLength,
    FileOutOfRange,
    RankOutOfRange,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Square(pub u8);

impl Square {
    /// Create a square from a string
    #[must_use]
    pub fn from_string(word: &str) -> Result<Square, ParseError> {
        if word.len() != 2 {
            Err(ParseError::IllegalLength)
        } else {
            if word.chars().nth(0).unwrap() < 'a' || word.chars().nth(0).unwrap() > 'h' {
                return Err(ParseError::FileOutOfRange);
            }

            if word.chars().nth(1).unwrap() < '1' || word.chars().nth(1).unwrap() > '8' {
                return Err(ParseError::RankOutOfRange);
            }

            let x = word.chars().nth(0).unwrap() as u8 - b'a';
            let y = word.chars().nth(1).unwrap() as u8 - b'1';

            Ok(Self::from_file_rank(x, y))
        }
    }

    /// Create a square from x and y coordinates
    #[must_use]
    pub fn from_file_rank(x: u8, y: u8) -> Self {
        Self(y * 8 + x)
    }

    /// Get the x coordinate
    #[must_use]
    pub fn get_x(&self) -> u8 {
        self.0 % 8
    }

    /// Get the y coordinate
    #[must_use]
    pub fn get_y(&self) -> u8 {
        self.0 / 8
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            ('a' as u8 + self.get_x()) as char,
            ('1' as u8 + self.get_y()) as char
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        // Pass
        assert!(Square::from_string("a1").is_ok());
        assert!(Square::from_string("a8").is_ok());
        assert!(Square::from_string("h1").is_ok());
        assert!(Square::from_string("h8").is_ok());
        // Fail
        assert_eq!(Square::from_string("1"), Err(ParseError::IllegalLength));
        assert_eq!(Square::from_string("123"), Err(ParseError::IllegalLength));
        assert_eq!(Square::from_string("a0"), Err(ParseError::RankOutOfRange));
        assert_eq!(Square::from_string("a9"), Err(ParseError::RankOutOfRange));
        assert_eq!(Square::from_string("i1"), Err(ParseError::FileOutOfRange));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Square::from_string("a1").unwrap().to_string(), "a1");
        assert_eq!(Square::from_string("h1").unwrap().to_string(), "h1");
        assert_eq!(Square::from_string("a8").unwrap().to_string(), "a8");
        assert_eq!(Square::from_string("h8").unwrap().to_string(), "h8");
    }

    #[test]
    fn test_coordinates() {
        for x in 0..8 {
            for y in 0..8 {
                let sq = Square::from_file_rank(x, y);
                assert_eq!(sq.get_x(), x, "Square x coordinate mismatch");
                assert_eq!(sq.get_y(), y, "Square y coordinate mismatch");
            }
        }
    }
}
