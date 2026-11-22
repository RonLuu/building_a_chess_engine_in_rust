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
    let pos = Position::new_with_fen("4k3/8/8/8/8/8/r6r/R1P1K1PR w KQ - 0 1");
    println!("{}", pos);
    let generated_move = pos.generate_legal_moves();
    let mut counter = 0;
    for chess_move in generated_move {
        println!("{} {}", chess_move, counter);
        counter += 1;
    }

}
