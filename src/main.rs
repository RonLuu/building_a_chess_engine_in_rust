mod piece;
mod board;
mod game_state;
mod position;
mod chess_move;
mod move_gen;
mod attack;
mod util;
mod constants;
mod perft;


use crate::{perft::{perft, perft_divide}, position::Position};

fn main() {
    let pos = Position::new();
    let _pos = Position::new_with_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");
    println!("{}",pos);
    // println!("{}", chess_moves_to_string(&pos.generate_legal_moves()))
    // for i in 1..5 {
    //     let count = perft(pos, i);
    //     println!("{}", count);
    // }
    perft_divide(pos, 2);
}
