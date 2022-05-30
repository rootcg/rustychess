use std::str::FromStr;

pub fn parse_capture(c: &str) -> bool {
    "x" == c
}

#[derive(PartialEq, Debug)]
pub enum CheckType {
    Check,
    Checkmate,
}

#[derive(PartialEq, Debug)]
pub enum ParseCheckTypeErr {
    EmptyString,
    UnknownCheck
}

impl FromStr for CheckType {
    type Err = ParseCheckTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseCheckTypeErr::EmptyString)
        }

        match s {
            "+" => Ok(CheckType::Check),
            "#" => Ok(CheckType::Checkmate),
            _ => Err(ParseCheckTypeErr::UnknownCheck)
        }
    }
}
