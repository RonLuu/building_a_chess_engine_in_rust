use crate::{chess_move::Flag};

pub fn check_sq_in_board(rank: i8, file: i8) -> bool {
    (0 <= rank && rank < 8) && (0 <= file && file < 8)
}

// pub fn rank_and_file_to_square(rank: u8, file: u8) -> u8 {
//     rank * 8 + file
// }

pub fn square_to_string(square: u8) -> String {
    let mut str_square: String = String::new();
    str_square.push((square % 8 + b'a') as char);
    str_square.push((square / 8 + b'1') as char);
    str_square
}

pub fn flag_to_string(flag: Option<Flag>) -> String {
    match flag {
        None => String::new(),
        Some(Flag::EnPassant) => String::from("En Passant move"),
        Some(Flag::Castle) => String::from("Castle move"),
        Some(Flag::Promotion(piece_type)) => piece_type.to_string(),
    }
}
