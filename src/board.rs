use std::fmt;

use crate::piece::Piece;
use crate::piece::PieceType;
use crate::piece::Color;

const FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


pub struct ChessBoard {
    board: [[Option<Piece>; 8]; 8],
    side: Color,
    castle_right: u8,
    en_pass_sq: u8,
    half_move: u8,
    full_move: u16,
}

impl ChessBoard {
  
    pub fn new() -> Self {
        Self::new_with_fen(FEN)
    }
    
    pub fn new_with_fen(fen: &str) -> Self {
        let byte_fen:&[u8]  = fen.as_bytes();
        let mut byte_index = 0;

        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        Self::parse_board(&mut board, byte_fen, &mut byte_index);

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
            board,
            side,
            castle_right,
            en_pass_sq,
            half_move,
            full_move,
        }
    }
    
    
    fn parse_board(board: &mut [[Option<Piece>; 8]; 8], byte_fen: &[u8], byte_index: &mut usize) {
        let mut board_index: usize = 0;
        
        while board_index < 64 { 
            if byte_fen[*byte_index] == b'/' {
                
            } else if byte_fen[*byte_index].is_ascii_digit() {
                let num_of_empty = (byte_fen[*byte_index] - b'0') as usize;
                board_index += num_of_empty
            } else {
                let c = byte_fen[*byte_index] as char;
                board[(8-1) - board_index/8][board_index%8] = Some(Self::get_piece_from_char(c));
                board_index += 1
            }
            *byte_index += 1;
        }
        *byte_index += 1;
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

    fn get_piece_from_char(c: char) -> Piece {
        match c {
            'r' => Piece {piece_type: PieceType::Rook, color: Color::Black},
            'R' => Piece {piece_type: PieceType::Rook, color: Color::White},
            
            'b' => Piece {piece_type: PieceType::Bishop, color: Color::Black},
            'B' => Piece {piece_type: PieceType::Bishop, color: Color::White},
            
            'n' => Piece {piece_type: PieceType::Knight, color: Color::Black},
            'N' => Piece {piece_type: PieceType::Knight, color: Color::White},
            
            'p' => Piece {piece_type: PieceType::Pawn, color: Color::Black},
            'P' => Piece {piece_type: PieceType::Pawn, color: Color::White},
            
            'q' => Piece {piece_type: PieceType::Queen, color: Color::Black},
            'Q' => Piece {piece_type: PieceType::Queen, color: Color::White},
            
            'k' => Piece {piece_type: PieceType::King, color: Color::Black},
            'K' => Piece {piece_type: PieceType::King, color: Color::White},

            _ => panic!("Illegal FEN: Wrong character piece")
        }
    }

}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board = &self.board;
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

            if row + 1 == 6 {
                board_string.push_str("   Side: ");
                board_string.push_str(&(self.side.to_string()));
            }

            if row + 1 == 5 {
                board_string.push_str("   Castle rights: ");
                board_string.push_str(&self.castle_right.to_string());
            }
            
            if row + 1 == 4 {
                board_string.push_str("   En passant square: ");
                board_string.push_str(&self.en_pass_sq.to_string());                
            }
            
            if row + 1 == 3 {
                board_string.push_str("   Half moves: ");
                board_string.push_str(&self.half_move.to_string());
            }
            
            if row + 1 == 2 {
                board_string.push_str("   Full moves: ");
                board_string.push_str(&self.full_move.to_string());
            }

            board_string.push_str("\n");
        }
        
        board_string.push_str("   ");
        for col in 0..8 {
            let file = col + 97 as u8;
            board_string.push_str(&(file as char).to_string());
            board_string.push_str(" ");
        }
        
        write!(f, "{}", board_string)
    }   
}