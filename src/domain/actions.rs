use std::{io::Error, str::FromStr};

pub fn parse_capture(c: char) -> bool {
    'x' == c
}

#[derive(PartialEq, Debug)]
pub enum CheckType {
    Check,
    Checkmate,
}

impl FromStr for CheckType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
        Ok(CheckType::Checkmate)
    }
}
