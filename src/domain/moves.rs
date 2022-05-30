use super::{
    actions::CheckType,
    piece::Piece,
    positions::{Destiny, PositionParseErr, Source},
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

static MOVE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::from_str("^([KQBNR]?)([abcdefgh][012345678]?)?(x?)([abcdefgh][012345678])([+#]?)$").unwrap()
});

#[derive(PartialEq, Debug)]
pub struct Move {
    piece: Option<Piece>,
    source: Option<Source>,
    capture: bool,
    destiny: Destiny,
    check: Option<CheckType>,
}

impl FromStr for Move {
    type Err = MoveParseErr;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(MoveParseErr::EmptyString);
        }

        if input.len() < 2 {
            return Err(MoveParseErr::IncorrectStringLength);
        }

        if !MOVE_REGEX.is_match(input) {
            return Err(MoveParseErr::IncorrectFormat);
        }

        let groups_captured = MOVE_REGEX.captures(input).unwrap();
        let piece = groups_captured.get(1).map(|x| x.as_str()).map(Piece::from_str);
        let source = groups_captured.get(2).map(|x| x.as_str());
        let capture = groups_captured.get(3).map(|x| x.as_str());
        let destiny = groups_captured.get(4).map(|x| x.as_str());
        let check = groups_captured.get(5).map(|x| x.as_str());
        
        todo!()
    }
}

#[derive(PartialEq, Debug)]
pub enum MoveParseErr {
    EmptyString,
    IncorrectStringLength,
    IncorrectFormat,
    IncorrectPosition(PositionParseErr),
}

impl From<PositionParseErr> for MoveParseErr {
    fn from(e: PositionParseErr) -> Self {
        MoveParseErr::IncorrectPosition(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(Err(MoveParseErr::EmptyString), Move::from_str(""))
    }

    #[test]
    fn small_string() {
        assert_eq!(
            Err(MoveParseErr::IncorrectStringLength),
            Move::from_str("f")
        )
    }

    #[test]
    fn incorrect_format() {
        assert_eq!(Err(MoveParseErr::IncorrectFormat), Move::from_str("a3x43"))
    }
}
