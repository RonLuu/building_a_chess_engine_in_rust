use crate::{Position};

pub fn perft(position: Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut count = 0;
    let legal_moves = position.generate_legal_moves();
    for legal_move in legal_moves {
        let new_position = position.apply_move(&legal_move);
        count += perft(new_position, depth - 1);
    }

    return count;
}

pub fn perft_divide(position: Position, depth: u8) {
    let legal_moves = position.generate_legal_moves();

    for legal_move in legal_moves {
        let new_position = position.apply_move(&legal_move);
        let count = perft(new_position, depth - 1);
        println!("{}:{}", legal_move, count)
    }
}