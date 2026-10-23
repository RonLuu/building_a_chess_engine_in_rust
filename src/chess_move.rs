use std::fmt;

use crate::{piece::{self, PieceType}};

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
    from: u8,
    /// To square index (rank * 8 + file)
    to: u8,
    /// 'None' for ordinary move
    /// 'Some' for promotion, EnPassant and Castle
    flag: Option<Flag>
}

impl ChessMove {
    pub fn new(from: u8, to: u8, flag: Option<Flag>) -> Self {
        Self {
            from,
            to,
            flag
        }
    }

    pub fn rank_and_file_to_square(rank: u8, file: u8) -> u8 {
        rank * 8 + file
    }
    fn square_to_string(square: u8) -> String{
        let mut str_square: String = String::new();
        str_square.push((square%8 + b'a') as char);
        str_square.push((square/8 + b'1') as char);
        str_square
    }
    
    fn flag_to_string(flag: Option<Flag>) -> String{
        match flag {
            None => String::new(),
            Some(Flag::EnPassant) => String::from("En Passant move"),
            Some(Flag::Castle) => String::from("Castle move"),
            Some(Flag::Promotion(piece_type)) => piece_type.to_string(),
        }
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_chess_move: String = String::new();
        str_chess_move.push_str(&(Self::square_to_string(self.from)));
        str_chess_move.push_str("->");
        str_chess_move.push_str(&(Self::square_to_string(self.to)));
        str_chess_move.push_str("  ");
        str_chess_move.push_str(&(Self::flag_to_string(self.flag)));

        write!(f, "{}", str_chess_move)
    }
}
