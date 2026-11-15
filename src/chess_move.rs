use std::fmt;

use crate::{piece::{PieceType}, util::{square_to_string, flag_to_string}};

/// Additional flag for a move, can only be
/// Promotion
/// EnPassant
/// Castle
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    Promotion(PieceType),
    EnPassant,
    Castle,
}

/// A single move from one square to another, plus any special flag needed
/// to apply it correctly.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChessMove {
    /// From square index (rank * 8 + file)
    pub from: u8,
    /// To square index (rank * 8 + file)
    pub to: u8,
    /// 'None' for ordinary move
    /// 'Some' for promotion, EnPassant and Castle
    pub flag: Option<Flag>
}

impl ChessMove {
    pub fn new(from: u8, to: u8, flag: Option<Flag>) -> Self {
        Self {
            from,
            to,
            flag
        }
    }

}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_chess_move: String = String::new();
        str_chess_move.push_str(&(square_to_string(self.from)));
        str_chess_move.push_str("->");
        str_chess_move.push_str(&(square_to_string(self.to)));
        str_chess_move.push_str("  ");
        str_chess_move.push_str(&(flag_to_string(self.flag)));

        write!(f, "{}", str_chess_move)
    }
}
