use std::str::FromStr;

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
pub enum ParseSourceErr {
    EmptyString,
    UnknownRow,
    UnknownColumn,
    UnknownPosition
}

impl FromStr for Source {
    type Err = ParseSourceErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseSourceErr::EmptyString)
        }

        if s.len() > 2 {
            return Err(ParseSourceErr::UnknownPosition)
        }

        let (column, row) = s.split_at(1);
        let column = match column.chars().next() {
            Some(c @ 'a'..='h') => c,
            _ => return Err(ParseSourceErr::UnknownColumn)
        };

        let row = match row {
            _ if row.is_empty() => None,
            _ => match row.parse::<u8>() {
                Ok(r @ 1..=8) => Some(r),
                _ => return Err(ParseSourceErr::UnknownRow) 
            }
        };

        Ok(Source { row, column })
    }
}

impl FromStr for Destiny {
    type Err = ParseSourceErr;

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
        assert_eq!(Err(ParseSourceErr::EmptyString), Source::from_str(""))
    }

    #[test]
    fn parse_source_unknown_position() {
        assert_eq!(Err(ParseSourceErr::UnknownPosition), Source::from_str("ab321"))
    }
    
    #[test]
    fn parse_source_unknown_row() {
        assert_eq!(Err(ParseSourceErr::UnknownRow), Source::from_str("ax"));
        assert_eq!(Err(ParseSourceErr::UnknownRow), Source::from_str("a9"));
    }

    #[test]
    fn parse_source_unknown_column() {
        assert_eq!(Err(ParseSourceErr::UnknownColumn), Source::from_str("98"));
        assert_eq!(Err(ParseSourceErr::UnknownColumn), Source::from_str("k8")); 
    }

    #[test]
    fn parse_source_only_column() {
        assert_eq!(Source { row: None, column: 'a' }, Source::from_str("a").unwrap());
    }
    
    #[test]
    fn parse_source_with_row_and_column() {
        assert_eq!(Source { row: Some(2), column: 'h' }, Source::from_str("h2").unwrap());
    }
}
