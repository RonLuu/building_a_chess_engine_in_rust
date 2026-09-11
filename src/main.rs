mod piece;
mod board;

use crate::board::ChessBoard;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let chess_board: ChessBoard = board::ChessBoard::new_with_fen(fen);
    println!("{}", chess_board)
    
}
