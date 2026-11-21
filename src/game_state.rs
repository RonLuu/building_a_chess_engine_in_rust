use std::fmt;

use crate::{
    constants::{
        BLACK_KING_CASTLE_BIT, BLACK_QUEEN_CASTLE_BIT, ROOK_SQUARES, WHITE_KING_CASTLE_BIT,
        WHITE_QUEEN_CASTLE_BIT,
    },
    piece::{Color, Piece, PieceType},
};

pub const EMPTY_EN_PASS_SQ: u8 = 64;

#[derive(Debug, Clone)]
pub struct GameState {
    pub side: Color,
    pub castle_right: u8,
    pub en_pass_sq: u8,
    pub half_move: u8,
    pub full_move: u16,
    pub w_king_sq: u8,
    pub b_king_sq: u8,
}

impl GameState {
    pub fn new_with_fen(byte_fen: &[u8], w_king_sq: u8, b_king_sq: u8) -> Self {
        let mut byte_index = 0;

        let mut side: Color = Color::White;
        Self::parse_side(&mut side, byte_fen, &mut byte_index);

        let mut castle_right: u8 = 0;
        Self::parse_castle_right(&mut castle_right, byte_fen, &mut byte_index);

        let mut en_pass_sq: u8 = EMPTY_EN_PASS_SQ;
        Self::parse_en_pass_sq(&mut en_pass_sq, byte_fen, &mut byte_index);

        let mut half_move: u8 = 0;
        let mut full_move: u16 = 0;
        Self::parse_half_and_full_moves(&mut half_move, &mut full_move, byte_fen, &mut byte_index);

        Self {
            side,
            castle_right,
            en_pass_sq,
            half_move,
            full_move,
            w_king_sq,
            b_king_sq,
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
            return;
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
            return;
        }

        let file_number = byte_fen[*byte_index] - b'a';
        *byte_index += 1;
        let rank_number = byte_fen[*byte_index] - b'1';
        *byte_index += 2;

        *en_pass_sq = rank_number * 8 + file_number;
    }

    fn parse_half_and_full_moves(
        half_move: &mut u8,
        full_move: &mut u16,
        byte_fen: &[u8],
        byte_index: &mut usize,
    ) {
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

    pub fn apply_move(&mut self, from_square: u8, to_square: u8, piece: Piece, is_captured: bool) {
        let color = piece.color;
        
        // Increment full move
        if color == Color::Black {
            self.full_move += 1;
        } 

        // Switch side
        self.update_side(color);
        
        // Reset en pass square
        self.en_pass_sq = 64;
        
        
        if is_captured || piece.piece_type == PieceType::Pawn {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }

        // Update king square
        if piece.piece_type == PieceType::King {
            self.update_king_square(to_square, color);
            // Remove castle right
            if color == Color::White {
                self.update_castle(WHITE_KING_CASTLE_BIT);
                self.update_castle(WHITE_QUEEN_CASTLE_BIT);
            } else {
                self.update_castle(BLACK_KING_CASTLE_BIT);
                self.update_castle(BLACK_QUEEN_CASTLE_BIT);
            }
        }

        // Update en passant
        if (piece.piece_type == PieceType::Pawn)
            && (from_square as i8 - to_square as i8).abs() == 16 {
            self.update_en_passant(to_square, color);
        }

        // If rook moves, 
        if piece.piece_type == PieceType::Rook {
            // Remove castle right
            if color == Color::White && from_square == ROOK_SQUARES[0] {
                self.update_castle(WHITE_QUEEN_CASTLE_BIT);
            }
            if color == Color::White && from_square == ROOK_SQUARES[1] {
                self.update_castle(WHITE_KING_CASTLE_BIT);
            }
            if color == Color::Black && from_square == ROOK_SQUARES[2] {
                self.update_castle(BLACK_QUEEN_CASTLE_BIT);
            }
            if color == Color::Black && from_square == ROOK_SQUARES[3] {
                self.update_castle(BLACK_KING_CASTLE_BIT);
            }
        }

        // If some thing moves to rook square
        // Remove castle right
        if to_square == ROOK_SQUARES[0] {
            self.update_castle(WHITE_QUEEN_CASTLE_BIT);
        }
        if to_square == ROOK_SQUARES[1] {
            self.update_castle(WHITE_KING_CASTLE_BIT);
        }
        if to_square == ROOK_SQUARES[2] {
            self.update_castle(BLACK_QUEEN_CASTLE_BIT);
        }
        if to_square == ROOK_SQUARES[3] {
            self.update_castle(BLACK_KING_CASTLE_BIT);
        }
    }

    fn update_side(&mut self, color: Color) {
        if color == Color::White {
            self.side = Color::Black;
        } else {
            self.side = Color::White;
        }
    }

    fn update_king_square(&mut self, new_king_sq: u8, color: Color) {
        if color == Color::White {
            self.w_king_sq = new_king_sq;
        } else {
            self.b_king_sq = new_king_sq;
        }
    }

    fn update_en_passant(&mut self, to_square: u8, color: Color) {
        if color == Color::White {
            self.en_pass_sq = to_square - 8;
        } else {
            self.en_pass_sq = to_square + 8;
        }
    }

    fn update_castle(&mut self, castle_right_bit: u8) {
        self.castle_right &= !castle_right_bit
    }
}

impl fmt::Display for GameState {
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
