use std::fmt;

use crate::piece::Color;

#[derive(Debug)]
pub struct GameState {
    side: Color,
    castle_right: u8,
    en_pass_sq: u8,
    half_move: u8,
    full_move: u16,
}

impl GameState {
    pub fn new_with_fen(byte_fen:&[u8]) -> Self {
        let mut byte_index = 0;

        let mut side: Color = Color::White;
        Self::parse_side(&mut side, byte_fen, &mut byte_index);

        let mut castle_right: u8 =  0;
        Self::parse_castle_right(&mut castle_right, byte_fen, &mut byte_index);

        let mut en_pass_sq: u8 =  64;
        Self::parse_en_pass_sq(&mut en_pass_sq, byte_fen, &mut byte_index);
        
        let mut half_move: u8 =  0;
        let mut full_move: u16 =  0;
        Self::parse_half_and_full_moves(&mut half_move, &mut full_move, byte_fen, &mut byte_index);
        
        Self { 
            side,
            castle_right,
            en_pass_sq,
            half_move,
            full_move,
        }
    }
    
    fn parse_side(side: &mut Color, byte_fen: &[u8], byte_index: &mut usize) {
        if byte_fen[*byte_index] == b'w' {
            *side = Color::White;
        } else {
            *side = Color::Black;
        }
        *byte_index += 2;
    }

    fn parse_castle_right(castle_right: &mut u8, byte_fen: &[u8], byte_index: &mut usize) {

        if byte_fen[*byte_index] == b'-' {
            *byte_index += 2;
            return
        }

        while byte_fen[*byte_index] != b' ' {
            match byte_fen[*byte_index] {
                b'K' => *castle_right |= 8,
                b'Q' => *castle_right |= 4,
                b'k' => *castle_right |= 2,
                b'q' => *castle_right |= 1,
                _ => panic!("Illegal FEN: Wrong castle character"),
            }
            *byte_index += 1;
        }
        
        *byte_index += 1;
    }
    
    fn parse_en_pass_sq(en_pass_sq: &mut u8, byte_fen: &[u8], byte_index: &mut usize) {

        if byte_fen[*byte_index] == b'-' {
            *byte_index += 2;
            return 
        }

        let file_number = byte_fen[*byte_index] - b'a';
        *byte_index += 1;
        let rank_number = byte_fen[*byte_index] - b'1';
        *byte_index += 2;
        
        *en_pass_sq = rank_number * 8 + file_number;
    }

    fn parse_half_and_full_moves(half_move: &mut u8, full_move: &mut u16, byte_fen: &[u8], byte_index: &mut usize) {

        while byte_fen[*byte_index] != b' ' {
            let digit = byte_fen[*byte_index] - b'0';
            *half_move = *half_move * 10 + digit;
            *byte_index += 1;
        }
        *byte_index += 1;

        while *byte_index < byte_fen.len() {
            let digit = byte_fen[*byte_index] - b'0';
            *full_move = *full_move * 10 + (digit as u16);
            *byte_index += 1;
        }
    }
}

impl fmt::Display for GameState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut game_state_string: String = String::new();
        game_state_string.push_str("\nSide: ");
        game_state_string.push_str(&(self.side.to_string()));

        game_state_string.push_str("\nCastle rights: ");
        game_state_string.push_str(&self.castle_right.to_string());
                
        game_state_string.push_str("\nEn passant square: ");
        game_state_string.push_str(&self.en_pass_sq.to_string());                
                
        game_state_string.push_str("\nHalf moves: ");
        game_state_string.push_str(&self.half_move.to_string());
                
        game_state_string.push_str("\nFull moves: ");
        game_state_string.push_str(&self.full_move.to_string());

        write!(f, "{}", game_state_string)
    }

}
