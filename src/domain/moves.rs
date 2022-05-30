use super::{
    actions::{self, CheckType, ParseCheckTypeErr},
    piece::{ParsePieceErr, Piece},
    positions::{Destiny, ParsePositionErr, Source},
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

static MOVE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::from_str("^([KQBNR]?)([abcdefgh][012345678]?)?(x?)([abcdefgh][012345678])([+#]?)$")
        .unwrap()
});

#[derive(PartialEq, Debug)]
pub enum MoveParseErr {
    EmptyString,
    IncorrectFormat,
    MissingDestiny,
    IncorrectPosition(ParsePositionErr),
    IncorrectPiece(ParsePieceErr),
    IncorrectCheck(ParseCheckTypeErr),
}

impl From<ParsePositionErr> for MoveParseErr {
    fn from(e: ParsePositionErr) -> Self {
        MoveParseErr::IncorrectPosition(e)
    }
}

impl From<ParsePieceErr> for MoveParseErr {
    fn from(e: ParsePieceErr) -> Self {
        MoveParseErr::IncorrectPiece(e)
    }
}

impl From<ParseCheckTypeErr> for MoveParseErr {
    fn from(e: ParseCheckTypeErr) -> Self {
        MoveParseErr::IncorrectCheck(e)
    }
}

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

        if !MOVE_REGEX.is_match(input) {
            return Err(MoveParseErr::IncorrectFormat);
        }

        let groups_captured = MOVE_REGEX.captures(input).unwrap();
        let destiny = match groups_captured.get(4) {
            Some(m) => Destiny::from_str(m.as_str())?,
            _ => return Err(MoveParseErr::MissingDestiny),
        };

        let piece = groups_captured
            .get(1)
            .map(|x| x.as_str())
            .map(Piece::from_str)
            .transpose()?;
        let source = groups_captured
            .get(2)
            .map(|x| x.as_str())
            .map(Source::from_str)
            .transpose()?;
        let check = groups_captured
            .get(5)
            .map(|x| x.as_str())
            .map(CheckType::from_str)
            .transpose()?;
        let capture = groups_captured
            .get(3)
            .map(|x| x.as_str())
            .map(actions::parse_capture)
            .unwrap_or(false);

        Ok(Move {
            piece,
            source,
            capture,
            destiny,
            check,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string() {
        assert_eq!(Err(MoveParseErr::EmptyString), Move::from_str(""))
    }

    #[test]
    fn parse_incorrect_format() {
        assert_eq!(Err(MoveParseErr::IncorrectFormat), Move::from_str("a"));
        assert_eq!(Err(MoveParseErr::IncorrectFormat), Move::from_str("a3x43"));
    }
}
