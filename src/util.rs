pub fn check_sq_in_board(rank: i8, file: i8) -> bool {
    (0 <= rank && rank < 8) && (0 <= file && file < 8)
}