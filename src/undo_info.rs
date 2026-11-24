use crate::piece::Piece;

pub struct UndoInfo {
    pub castle_right: u8,
    pub en_pass_sq: u8,
    pub half_move: u8,
    pub captured_piece: Option<Piece>,
}

impl UndoInfo {
    pub fn new(
        castle_right: u8,
        en_pass_sq: u8,
        half_move: u8,
        captured_piece: Option<Piece>,
    ) -> Self {
        Self {
            castle_right,
            en_pass_sq,
            half_move,
            captured_piece,
        }
    }
}
