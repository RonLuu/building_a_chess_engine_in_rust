use std::fmt;

use crate::piece::Piece;

#[derive(Debug)]
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