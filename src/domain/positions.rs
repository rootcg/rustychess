use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Source {
    row: Option<u8>,
    column: char,
}

#[derive(PartialEq, Debug)]
pub struct Destiny {
    row: u8,
    column: char,
}

#[derive(PartialEq, Debug)]
pub enum PositionParseErr {
    EmptyString,
    EmptyRow,
    EmptyColumn,
    TooManyParameters,
}

impl FromStr for Source {
    type Err = PositionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
        Ok(Source {
            row: None,
            column: 'a',
        })
    }
}

impl FromStr for Destiny {
    type Err = PositionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
        Ok(Destiny {
            row: 1,
            column: 'a',
        })
    }
}
