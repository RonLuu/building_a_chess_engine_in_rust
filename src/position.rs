use std::fmt;

use crate::{board::Board, game_state::GameState };

const FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
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
        Self {
            board: Board::new_with_fen(board_fen),
            game_state: GameState::new_with_fen(game_state_fen)
        }
    }

    fn split_fen(fen: &str) -> (&[u8], &[u8]) {
        let mut byte_index: usize = 0;
        let byte_fen = fen.as_bytes();

        while byte_fen[byte_index] != b' ' {
            byte_index += 1
        }

        (&byte_fen[..byte_index], &byte_fen[byte_index+1..])
    }

}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}{}", self.board, self.game_state)
    }
}
