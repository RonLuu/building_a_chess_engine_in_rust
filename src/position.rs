use core::panic;
use std::fmt;

use crate::{
    board::Board,
    chess_move::{ChessMove, Flag},
    game_state::GameState,
    piece::{Color, Piece, PieceType},
    constants::{EN_PASSANT_RANKS, END_RANKS}
};

const FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone)]
pub struct Position {
    pub board: Board,
    pub game_state: GameState
}

impl Position {
    pub fn new() -> Self {
        Self::new_with_fen(FEN)
    }

    pub fn new_with_fen(fen: &str) -> Self {
        let (board_fen, game_state_fen) = Self::split_fen(fen);
        let (w_king_sq, b_king_sq) = Self::find_king_sq(board_fen);

        Self {
            board: Board::new_with_fen(board_fen),
            game_state: GameState::new_with_fen(game_state_fen, w_king_sq, b_king_sq),
        }
    }

    fn split_fen(fen: &str) -> (&[u8], &[u8]) {
        let mut byte_index: usize = 0;
        let byte_fen = fen.as_bytes();

        while byte_fen[byte_index] != b' ' {
            byte_index += 1
        }

        (&byte_fen[..byte_index], &byte_fen[byte_index + 1..])
    }

    fn find_king_sq(board_fen: &[u8]) -> (u8, u8) {
        let (mut w_king_sq, mut b_king_sq) = (64, 64);
        let mut byte_index: usize = 0;

        let mut col: u8 = 0;
        for rank in (0..=7).rev() {
            while byte_index < board_fen.len() && board_fen[byte_index] != b'/' {
                if board_fen[byte_index].is_ascii_digit() {
                    col += (board_fen[byte_index] - b'0') as u8;
                } else {
                    let c = board_fen[byte_index] as char;
                    let piece = Piece::get_piece_from_char(c);
                    if piece.piece_type == PieceType::King {
                        if piece.color == Color::White {
                            w_king_sq = rank * 8 + col;
                        } else {
                            b_king_sq = rank * 8 + col;
                        }
                    }
                    col += 1
                }
                byte_index += 1;
            }
            col = 0;
            byte_index += 1;
        }

        (w_king_sq, b_king_sq)
    }

    fn apply_move(&self, chess_move: &ChessMove) -> Position {
        let mut new_position = self.clone();
        let new_board = &mut new_position.board;
        let new_game_state = &mut new_position.game_state;

        let from_square = chess_move.from;
        let to_square = chess_move.to;
        let flag = chess_move.flag;
        
        let from_piece = match new_board.get_piece_from_square(from_square as usize) {
            None => panic!("There should be a piece here"),
            Some(piece) => piece
        };
        
        let is_captured = new_board.get_piece_from_square(to_square as usize).is_some();
        
        new_game_state.apply_move(from_square, to_square, from_piece, is_captured);
        new_board.apply_move(from_square, to_square, flag, from_piece);
        new_position
    }

}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}{}", self.board, self.game_state)
    }
}
