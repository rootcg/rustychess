use std::{str::FromStr, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub struct Source {
    column: char,
    row: Option<u8>,
}

#[derive(PartialEq, Debug)]
pub struct Destiny {
    column: char,
    row: u8,
}

#[derive(PartialEq, Debug)]
pub enum ParsePositionErr {
    EmptyString,
    EmptyRow,
    UnknownRow,
    UnknownColumn,
    UnknownPosition
}

impl FromStr for Source {
    type Err = ParsePositionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParsePositionErr::EmptyString)
        }

        if s.len() > 2 {
            return Err(ParsePositionErr::UnknownPosition)
        }

        let (column, row) = s.split_at(1);
        let column = match column.chars().next() {
            Some(c @ 'a'..='h') => c,
            _ => return Err(ParsePositionErr::UnknownColumn)
        };

        let row = match row {
            _ if row.is_empty() => None,
            _ => match row.parse::<u8>() {
                Ok(r @ 1..=8) => Some(r),
                _ => return Err(ParsePositionErr::UnknownRow) 
            }
        };

        Ok(Source { row, column })
    }
}

impl FromStr for Destiny {
    type Err = ParsePositionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
        Ok(Destiny {
            row: 1,
            column: 'a',
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_source_empty_string() {
        assert_eq!(Err(ParsePositionErr::EmptyString), Source::from_str(""))
    }
    
    #[test]
    fn parse_source_unknown_row() {
        assert_eq!(Err(ParsePositionErr::UnknownRow), Source::from_str("ax"));
        assert_eq!(Err(ParsePositionErr::UnknownRow), Source::from_str("a9"));
    }

    #[test]
    fn parse_source_unknown_column() {
        assert_eq!(Err(ParsePositionErr::UnknownColumn), Source::from_str("98"));
    }

    // TODO: add more tests 
}
