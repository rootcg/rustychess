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
            .filter(|x| !x.is_empty())
            .map(Piece::from_str)
            .transpose()?;
        let source = groups_captured
            .get(2)
            .map(|x| x.as_str())
            .filter(|x| !x.is_empty())
            .map(Source::from_str)
            .transpose()?;
        let check = groups_captured
            .get(5)
            .map(|x| x.as_str())
            .filter(|x| !x.is_empty())
            .map(CheckType::from_str)
            .transpose()?;
        let capture = groups_captured
            .get(3)
            .map(|x| x.as_str())
            .filter(|x| !x.is_empty())
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

    #[test]
    fn parse_full_move() {
        let expected = Move {
            piece: Some(Piece::Rook),
            source: Some(Source { column: 'a', row: Some(1) }),
            capture: true,
            destiny: Destiny { column: 'b', row: 1 },
            check: Some(CheckType::Checkmate)
        };

        assert_eq!(expected, Move::from_str("Ra1xb1#").unwrap());
    }

    #[test]
    fn parse_ambiguous_source() {
        let expected = Move {
            piece: Some(Piece::Rook),
            source: Some(Source { column: 'a', row: Some(1) }),
            capture: false,
            destiny: Destiny { column: 'b', row: 1 },
            check: None
        };

        assert_eq!(expected, Move::from_str("Ra1b1").unwrap());
    }

    #[test]
    fn parse_pawn_capture() {
        let expected = Move {
            piece: None,
            source: Some(Source { column: 'f', row: None }),
            capture: true,
            destiny: Destiny { column: 'e', row: 4 },
            check: None
        };

        assert_eq!(expected, Move::from_str("fxe4").unwrap());
    }

    #[test]
    fn parse_only_destiny() {
        let expected = Move {
            piece: None,
            source: None,
            capture: false,
            destiny: Destiny { column: 'a', row: 1 },
            check: None
        };

        assert_eq!(expected, Move::from_str("a1").unwrap());
    }
}
