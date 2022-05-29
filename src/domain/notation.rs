use std::str::FromStr;

use self::positions::PositionParseErr;

enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Tower,
    Pawn,
}

mod actions {
    use std::{io::Error, str::FromStr};
    
    pub fn parse_capture(c: char) -> bool {
        'x' == c
    }

    pub enum CheckType {
        Check,
        Checkmate,
    }

    impl FromStr for Capture {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!();
            Ok(Capture)
        }
    }

    impl FromStr for CheckType {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!();
            Ok(CheckType::Checkmate)
        }
    }
}

mod positions {
    use std::str::FromStr;

    pub struct Source {
        row: Option<u8>,
        column: char,
    }

    pub struct Destiny {
        row: u8,
        column: char,
    }

    pub enum PositionParseErr {
        EmptyString,
        EmptyRow,
        EmptyColumn,
        TooManyParameters
    }

    impl FromStr for Source {
        type Err = PositionParseErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!();
            Ok(Source { row: None, column: 'a' })
        }
    }

    impl FromStr for Destiny {
        type Err = PositionParseErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!();
            Ok(Destiny { row: 1, column: 'a' })
        }
    }
}

pub struct Move {
    piece: Option<Piece>,
    source: Option<positions::Source>,
    capture: actions::Capture,
    destiny: positions::Destiny,
    check: Option<actions::CheckType>,
}

pub enum MoveParseErr {
    EmptyString,
    IncorrectStringLength,
    IncorrectPosition(positions::PositionParseErr)
}

impl From<PositionParseErr> for MoveParseErr {
    fn from(e: PositionParseErr) -> Self {
        MoveParseErr::IncorrectPosition(e)
    }
}

impl FromStr for Move {
    type Err = MoveParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(MoveParseErr::EmptyString)
        }

        if s.len() < 2 {
            return Err(MoveParseErr::IncorrectStringLength)
        }

        let mut s = s.chars().rev();
        let check_type_str = s.next().unwrap();
        let check_type: Option<actions::CheckType> = match check_type_str.to_string().parse() {
            Ok(check) => Some(check),
            Err(_) => None
        };

        let mut destiny = s.next().unwrap().to_string();
        match check_type {
            Some(_) => destiny = format!("{}{}", s.next().unwrap(), destiny),
            None => destiny.push(check_type_str)
        }
        let destiny = destiny.parse()?;

        let mut _move = Move {
            piece: None, 
            source: None,
            capture: false,
            destiny,
            check: check_type
        };

        let capture: bool = s.next().map(actions::parse_capture).unwrap_or(false);

        // refactor with maps the other optionals

        todo!()
    }
}
