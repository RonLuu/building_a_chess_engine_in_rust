use std::fmt;

use crate::{
    piece::{Piece, PieceType, Color},
    chess_move::{Flag}
};

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new_with_fen(byte_fen: &[u8]) -> Self {
        let mut byte_index = 0;

        let mut squares: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        let mut board_index: usize = 0;

        while board_index < 64 { 
            if byte_fen[byte_index] == b'/' {
                
            } else if byte_fen[byte_index].is_ascii_digit() {
                let num_of_empty = (byte_fen[byte_index] - b'0') as usize;
                board_index += num_of_empty
            } else {
                let c = byte_fen[byte_index] as char;
                squares[(8-1) - board_index/8][board_index%8] = Some(Piece::get_piece_from_char(c));
                board_index += 1
            }
            byte_index += 1;
        }

        Self {
            squares
        }
    }

    pub fn get_piece(&self, rank: usize, file: usize) -> Option<Piece> {
        self.squares[rank][file]
    }

    pub fn get_piece_from_square(&self, square: usize) -> Option<Piece> {
        let rank = square / 8;
        let file = square % 8;

        Self::get_piece(&self, rank, file)
    }

    pub fn apply_move(&mut self, from_square: u8, to_square: u8, flag: Option<Flag>, piece: Piece) {
        
        let color = piece.color;
        // Remove the from piece
        self.place_piece_from_square(from_square as usize, None);
        
        if let Some(Flag::Promotion(promoted_piece_type)) = flag {
            self.handle_promotion(to_square, promoted_piece_type, color);
        } else {
            // Add the to piece
            self.place_piece_from_square(to_square as usize, Some(piece));

            if flag == Some(Flag::EnPassant) {
                self.handle_en_passant(from_square, to_square);
            } else if  flag == Some(Flag::Castle) {
                self.handle_castle(from_square, to_square, color);
            } 
        }
    }
    fn place_piece_from_square(&mut self, square: usize, piece: Option<Piece>) {
        let rank = square / 8;
        let file = square % 8;
        self.squares[rank][file] = piece;
    }

    fn handle_promotion(&mut self, to_square: u8, promoted_piece_type: PieceType, color: Color ) {
        let promoted_piece = Some(Piece { piece_type: promoted_piece_type, color});
        self.place_piece_from_square(to_square as usize, promoted_piece);
    }

    fn handle_en_passant(&mut self, from_square: u8, to_square: u8) {
        let from_rank = from_square / 8;
        // assert!(from_rank == EN_PASSANT_RANKS[0] || from_rank == EN_PASSANT_RANKS[1]);
        let to_file = to_square % 8;
        // Remove the enemy's pawn
        let enemy_square = from_rank * 8 + to_file;
        self.place_piece_from_square(enemy_square as usize, None);
    }

    fn handle_castle(&mut self, from_square: u8, to_square: u8, color: Color) {
        // assert!(from_rank == to_rank);
        // assert!(from_rank == END_RANKS[0] || from_rank == END_RANKS[1]);
        let from_rank = from_square / 8;
        let from_file = from_square % 8;
        let to_file = to_square % 8;

        // Right castle
        if from_file < to_file {
            // Remove the rook
            let rook_square = from_rank * 8 + (to_file + 1);
            self.place_piece_from_square(rook_square as usize, None);
            
            // Add the rook back
            let rook_new_square = from_rank * 8 + (to_file - 1);
            self.place_piece_from_square(
                rook_new_square as usize, 
                Some(Piece{piece_type: PieceType::Rook, color})
            );
        } else {
            // Remove the rook
            let rook_square = from_rank * 8 + (to_file - 2);
            self.place_piece_from_square(rook_square as usize, None);
            
            // Add the rook back
            let rook_new_square = from_rank * 8 + (to_file + 1);
            self.place_piece_from_square(
                rook_new_square as usize, 
                Some(Piece{piece_type: PieceType::Rook, color})
            );
        }
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board = &self.squares;
        let mut board_string: String = String::new();

        for row in (0..8).rev() {
            board_string.push_str(&(row+1).to_string());
            board_string.push_str("  ");
            for col in 0..8 {
                match board[row][col] {
                    Some(piece) => {
                        board_string.push_str(&(piece.to_string()));
                        board_string.push_str(" ");
                    },
                    None => board_string.push_str(". "),
                }
            }

            board_string.push_str("\n");
        }
        
        board_string.push_str("   ");
        for col in 0..8 {
            let file = col + 97 as u8;
            board_string.push_str(&(file as char).to_string());
            board_string.push_str(" ");
        }
        
        write!(f, "{}\n", board_string)
    }   
}