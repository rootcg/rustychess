use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

#[derive(PartialEq, Debug)]
pub enum ParsePieceErr {
    EmptyString,
    UnknownPiece,
}

impl FromStr for Piece {
    type Err = ParsePieceErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "K" => Ok(Piece::King),
            "Q" => Ok(Piece::Queen),
            "B" => Ok(Piece::Bishop),
            "N" => Ok(Piece::Knight),
            "R" => Ok(Piece::Rook),
            "P" => Ok(Piece::Pawn),
            _ if s.is_empty() => Err(ParsePieceErr::EmptyString),
            _ => Err(ParsePieceErr::UnknownPiece),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn parse_empty_string() {
        assert_eq!(Err(ParsePieceErr::EmptyString), Piece::from_str(""))
    }

    #[test]
    fn parse_unknown_piece() {
        assert_eq!(Err(ParsePieceErr::UnknownPiece), Piece::from_str("X"))
    }

    #[test]
    fn parse_pieces() {
        let pieces = HashMap::from([
            ("K", Piece::King),
            ("Q", Piece::Queen),
            ("B", Piece::Bishop),
            ("N", Piece::Knight),
            ("R", Piece::Rook),
            ("P", Piece::Pawn),
        ]);

        pieces.iter().for_each(|(k, v)| {
            assert_eq!(Piece::from_str(k).unwrap(), *v)
        });
    }
}
