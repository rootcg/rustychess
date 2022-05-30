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
pub enum ParsePositionErr {
    EmptyString,
    UnknownRow,
    UnknownColumn,
    UnknownPosition,
}

impl FromStr for Source {
    type Err = ParsePositionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParsePositionErr::EmptyString);
        }

        if s.len() > 2 {
            return Err(ParsePositionErr::UnknownPosition);
        }

        let (column, row) = s.split_at(1);
        let column = match column.chars().next() {
            Some(c @ 'a'..='h') => c,
            _ => return Err(ParsePositionErr::UnknownColumn),
        };

        let row = match row {
            _ if row.is_empty() => None,
            _ => match row.parse::<u8>() {
                Ok(r @ 1..=8) => Some(r),
                _ => return Err(ParsePositionErr::UnknownRow),
            },
        };

        Ok(Source { row, column })
    }
}

impl FromStr for Destiny {
    type Err = ParsePositionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParsePositionErr::EmptyString);
        }

        if s.len() != 2 {
            return Err(ParsePositionErr::UnknownPosition);
        }

        let (column, row) = s.split_at(1);
        let column = match column.chars().next() {
            Some(c @ 'a'..='h') => c,
            _ => return Err(ParsePositionErr::UnknownColumn),
        };

        let row = match row.parse::<u8>() {
            Ok(r @ 1..=8) => r,
            _ => return Err(ParsePositionErr::UnknownRow),
        };

        Ok(Destiny { row, column })
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
    fn parse_source_unknown_position() {
        assert_eq!(
            Source::from_str("ab321"),
            Err(ParsePositionErr::UnknownPosition)
        )
    }

    #[test]
    fn parse_source_unknown_row() {
        assert_eq!(Source::from_str("ax"), Err(ParsePositionErr::UnknownRow));
        assert_eq!(Source::from_str("a9"), Err(ParsePositionErr::UnknownRow));
    }

    #[test]
    fn parse_source_unknown_column() {
        assert_eq!(Source::from_str("98"), Err(ParsePositionErr::UnknownColumn));
        assert_eq!(Source::from_str("k8"), Err(ParsePositionErr::UnknownColumn));
    }

    #[test]
    fn parse_source_only_column() {
        assert_eq!(
            Source {
                row: None,
                column: 'a'
            },
            Source::from_str("a").unwrap()
        );
    }

    #[test]
    fn parse_source_with_row_and_column() {
        assert_eq!(
            Source {
                row: Some(2),
                column: 'h'
            },
            Source::from_str("h2").unwrap()
        );
    }

    #[test]
    fn parse_source_whitespaces() {
        assert_eq!(
            Source {
                row: Some(3),
                column: 'a'
            },
            Source::from_str("  a3  ").unwrap()
        )
    }

    #[test]
    fn parse_destiny_empty_string() {
        assert_eq!(Err(ParsePositionErr::EmptyString), Destiny::from_str(""))
    }

    #[test]
    fn parse_destiny_unknown_position() {
        assert_eq!(
            Destiny::from_str("ab321"),
            Err(ParsePositionErr::UnknownPosition)
        )
    }

    #[test]
    fn parse_destiny_unknown_row() {
        assert_eq!(Destiny::from_str("ax"), Err(ParsePositionErr::UnknownRow));
        assert_eq!(Destiny::from_str("a9"), Err(ParsePositionErr::UnknownRow));
    }

    #[test]
    fn parse_destiny_unknown_column() {
        assert_eq!(
            Destiny::from_str("98"),
            Err(ParsePositionErr::UnknownColumn)
        );
        assert_eq!(
            Destiny::from_str("k8"),
            Err(ParsePositionErr::UnknownColumn)
        );
    }

    #[test]
    fn parse_destiny_only_column() {
        assert_eq!(Err(ParsePositionErr::UnknownPosition), Destiny::from_str("a"));
    }

    #[test]
    fn parse_destiny_with_row_and_column() {
        assert_eq!(
            Destiny {
                row: 2,
                column: 'h'
            },
            Destiny::from_str("h2").unwrap()
        );
    }

    #[test]
    fn parse_destiny_whitespaces() {
        assert_eq!(
            Destiny {
                row: 3,
                column: 'a'
            },
            Destiny::from_str("  a3  ").unwrap()
        )
    }
}
