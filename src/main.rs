mod piece;
mod board;
mod game_state;
mod position;
mod chess_move;
mod move_gen;
mod attack;
mod util;
mod constants;


use crate::{position::Position, piece::Color, util::square_to_string};

fn main() {
    let pos = Position::new_with_fen("4r2k/8/8/8/8/8/4B3/4K3 w - - 0 1");
    println!("{}", pos);
    let generated_move = pos.generate_pseudo_legal_moves();
    let mut counter = 0;
    for chess_move in generated_move {
        println!("{} {}", chess_move, counter);
        counter += 1;
    }

}
