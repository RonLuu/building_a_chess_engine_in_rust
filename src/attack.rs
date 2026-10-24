use crate::{
    board::Board,
    piece::{Color, PieceType},
    util::check_sq_in_board,
};

impl Board {
    pub fn is_square_attacked(&self, square: u8, attacking_color: Color) -> bool {
        Self::is_attacked_by_rook(&self, square, attacking_color)
            || Self::is_attacked_by_pawn(&self, square, attacking_color)
            || Self::is_attacked_by_queen(&self, square, attacking_color)
            || Self::is_attacked_by_bishop(&self, square, attacking_color)
            || Self::is_attacked_by_knight(&self, square, attacking_color)
    }

    fn is_attacked_by_knight(&self, square: u8, attacking_color: Color) -> bool {
        let knight_directions: [(i8, i8); 8] = [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ];

        let current_rank = square / 8;
        let current_file = square % 8;

        for direction in knight_directions {
            let new_rank: i8 = (current_rank as i8) + direction.0;
            let new_file: i8 = (current_file as i8) + direction.1;

            if check_sq_in_board(new_rank, new_file) {
                
                match self.get_piece(new_rank as usize, new_file as usize) {
                    None => {}
                    Some(piece) => {
                        if piece.piece_type == PieceType::Knight && piece.color == attacking_color {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }

    fn is_attacked_by_pawn(&self, square: u8, attacking_color: Color) -> bool {
        let pawn_direction: i8 = if attacking_color == Color::Black {
            1
        } else {
            -1
        };

        let current_rank = square / 8;
        let current_file = square % 8;
        let new_rank = current_rank as i8 + pawn_direction;

        for file_offset in [-1, 1] {
            let new_file = current_file as i8 + file_offset;
            if check_sq_in_board(new_rank, new_file) {
                if let Some(piece) = self.get_piece(new_rank as usize, new_file as usize) {
                    if piece.piece_type == PieceType::Pawn && piece.color == attacking_color {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_attacked_by_queen(&self, square: u8, attacking_color: Color) -> bool {
        let queen_directions: [(i8, i8); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
        ];
        return Self::is_attacked_by_sliding_piece(
            &self,
            square,
            attacking_color,
            PieceType::Queen,
            &queen_directions,
        );
    }

    fn is_attacked_by_rook(&self, square: u8, attacking_color: Color) -> bool {
        let rook_directions: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        return Self::is_attacked_by_sliding_piece(
            &self,
            square,
            attacking_color,
            PieceType::Rook,
            &rook_directions,
        );
    }

    fn is_attacked_by_bishop(&self, square: u8, attacking_color: Color) -> bool {
        let bishop_directions: [(i8, i8); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
        return Self::is_attacked_by_sliding_piece(
            &self,
            square,
            attacking_color,
            PieceType::Bishop,
            &bishop_directions,
        );
    }

    fn is_attacked_by_sliding_piece(
        &self,
        square: u8,
        attacking_color: Color,
        piece_type: PieceType,
        directions: &[(i8, i8)],
    ) -> bool {
        let current_rank = square / 8;
        let current_file = square % 8;

        for direction in directions {
            let mut new_rank = current_rank as i8 + direction.0;
            let mut new_file = current_file as i8 + direction.1;

            while check_sq_in_board(new_rank, new_file) {
                match self.get_piece(new_rank as usize, new_file as usize) {
                    None => {
                        new_rank += direction.0;
                        new_file += direction.1;
                    }
                    Some(piece) => {
                        if piece.piece_type == piece_type && piece.color == attacking_color {
                            return true;
                        }

                        return false;
                    }
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::{piece::Color, position::Position};

    #[test]
    fn starting_position_e1_not_attacked_by_black() {
        let pos = Position::new();
        assert!(!pos.board.is_square_attacked(4, Color::Black)); // e1 = 4
    }

    #[test]
    fn starting_position_e8_not_attacked_by_white() {
        let pos = Position::new();
        assert!(!pos.board.is_square_attacked(60, Color::White)); // e8 = 60
    }

    #[test]
    fn rook_attacks_along_clear_file() {
        // White rook on e1, nothing in between, black king on e8 — e8 should be attacked.
        let pos = Position::new_with_fen("4k3/8/8/8/8/8/8/4R3 w - - 0 1");
        assert!(pos.board.is_square_attacked(60, Color::White)); // e8
    }

    #[test]
    fn rook_attack_blocked_by_piece_in_between() {
        // Same as above, but a pawn on e4 blocks the file.
        let pos = Position::new_with_fen("4k3/8/8/8/4P3/8/8/4R3 w - - 0 1");
        assert!(!pos.board.is_square_attacked(60, Color::White)); // e8
    }

    #[test]
    fn bishop_attacks_along_clear_diagonal() {
        // White bishop on c1, clear diagonal to h6.
        let pos = Position::new_with_fen("8/8/7k/8/8/8/8/2B5 w - - 0 1");
        assert!(pos.board.is_square_attacked(47, Color::White)); // h6 = 47
    }

    #[test]
    fn knight_attacks_all_legal_l_squares() {
        // White knight on e4 (28). Legal targets: d6,f6,c5,g5,c3,g3,d2,f2.
        let pos = Position::new_with_fen("8/8/8/8/4N3/8/8/8 w - - 0 1");
        let targets = [43, 45, 34, 38, 18, 22, 11, 13]; // d6,f6,c5,g5,c3,g3,d2,f2
        for sq in targets {
            assert!(
                pos.board.is_square_attacked(sq, Color::White),
                "square {sq} should be attacked"
            );
        }
    }

    #[test]
    fn knight_does_not_attack_adjacent_squares() {
        let pos = Position::new_with_fen("8/8/8/8/4N3/8/8/8 w - - 0 1");
        let non_targets = [27, 29, 36, 20]; // d4, f4, e5, e3 (orthogonal/diagonal neighbors)
        for sq in non_targets {
            assert!(
                !pos.board.is_square_attacked(sq, Color::White),
                "square {sq} should NOT be attacked"
            );
        }
    }

    #[test]
    fn white_pawn_attacks_diagonals_forward() {
        // White pawn on e4 (28). Attacks d5 (35) and f5 (37).
        let pos = Position::new_with_fen("8/8/8/8/4P3/8/8/8 w - - 0 1");
        assert!(pos.board.is_square_attacked(35, Color::White)); // d5
        assert!(pos.board.is_square_attacked(37, Color::White)); // f5
    }

    #[test]
    fn white_pawn_does_not_attack_backward() {
        // Same pawn on e4 — must NOT attack d3 (19) or f3 (21), the backward diagonals.
        let pos = Position::new_with_fen("8/8/8/8/4P3/8/8/8 w - - 0 1");
        assert!(!pos.board.is_square_attacked(19, Color::White)); // d3
        assert!(!pos.board.is_square_attacked(21, Color::White)); // f3
    }

    #[test]
    fn black_pawn_attacks_diagonals_forward() {
        // Black pawn on e5 (36). Attacks d4 (27) and f4 (29) — forward for Black is toward rank 1.
        let pos = Position::new_with_fen("8/8/8/4p3/8/8/8/8 w - - 0 1");
        assert!(pos.board.is_square_attacked(27, Color::Black)); // d4
        assert!(pos.board.is_square_attacked(29, Color::Black)); // f4
    }
}
