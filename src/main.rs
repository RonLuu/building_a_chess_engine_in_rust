mod piece;
mod board;
mod game_state;
mod position;
mod chess_move;
mod move_gen;
mod attack;
mod util;
mod constants;


use crate::{position::Position};

fn main() {
    let _pos = Position::new();
    let pos = Position::new_with_fen("r3k2r/R6R/8/6B1/8/8/1R6/4K3 b kq - 0 1");
    println!("{}", pos);
    let generated_move = pos.generate_legal_moves();
    let mut counter = 0;
    for chess_move in generated_move {
        println!("{} {}", chess_move, counter);
        counter += 1;
    }

}
