use crate::{board::Board, chess_move::{ChessMove, Flag}, piece::{Color, PieceType}, position::Position};



impl Position {
    fn generate_pseudo_legal_moves(&self) -> Vec<ChessMove> {
        let mut pseudo_legal_moves: Vec<ChessMove> = Vec::new();
        let board: &Board = &(self.board);
        let side: Color = self.game_state.side;
        Self::generate_pseudo_legal_non_sliding_moves(side, board, &mut pseudo_legal_moves);
        // Self::generate_pseudo_legal_sliding_moves();
        pseudo_legal_moves
    }

    fn generate_pseudo_legal_non_sliding_moves(
        side: Color,
        board: &Board,
        pseudo_legal_moves: &mut Vec<ChessMove>,
    ) {
        Self::generate_pawn_moves(side, board, pseudo_legal_moves);
        Self::generate_king_moves(side, board, pseudo_legal_moves);
        Self::generate_knight_moves(side, board, pseudo_legal_moves);
    }

    fn generate_knight_moves(side: Color, board: &Board, pseudo_legal_moves: &mut Vec<ChessMove>) {
        let knight_directions: [(i8, i8); 8] = [
            (2, -1),
            (2, 1),
            (1, -2),
            (1, 2),
            (-1, 2),
            (-1, -2),
            (-2, 1),
            (-2, -1),
        ];

        for rank in 0..8 {
            for file in 0..8 {
                match board.get_piece(rank, file) {
                    Some(piece) => {
                        if piece.piece_type != PieceType::Knight {
                            continue;
                        }
                        if piece.color != side {
                            continue;
                        }

                        let from_sq: u8 = (rank * 8 + file) as u8;
                        for knight_direction in knight_directions {
                            let new_rank = rank as i8 + knight_direction.0;
                            let new_file = file as i8 + knight_direction.1;

                            if !Self::check_sq_in_board(new_rank, new_file) {
                                continue;
                            }

                            if Self::check_ally_piece(board, side, new_rank, new_file) {
                                continue;
                            }

                            let to_sq: u8 = (new_rank * 8 + new_file) as u8;
                            let chess_move = ChessMove::new(from_sq, to_sq, None);
                            pseudo_legal_moves.push(chess_move);
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    fn generate_king_moves(side: Color, board: &Board, pseudo_legal_moves: &mut Vec<ChessMove>) {
        let king_directions: [(i8, i8); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
        ];

        for rank in 0..8 {
            for file in 0..8 {
                match board.get_piece(rank, file) {
                    Some(piece) => {
                        if piece.piece_type != PieceType::King {
                            continue;
                        }
                        if piece.color != side {
                            continue;
                        }
                        let from_sq: u8 = (rank * 8 + file) as u8;
                        for king_direction in king_directions {
                            let new_rank = rank as i8 + king_direction.0;
                            let new_file = file as i8 + king_direction.1;

                            if !Self::check_sq_in_board(new_rank, new_file) {
                                continue;
                            }

                            if Self::check_ally_piece(board, side, new_rank, new_file) {
                                continue;
                            }

                            let to_sq: u8 = (new_rank * 8 + new_file) as u8;
                            let chess_move = ChessMove::new(from_sq, to_sq, None);
                            pseudo_legal_moves.push(chess_move);
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    fn generate_pawn_moves(side: Color, board: &Board, pseudo_legal_moves: &mut Vec<ChessMove>) {
        fn generate_pawn_starting_move(
            board: &Board,
            pseudo_legal_moves: &mut Vec<ChessMove>,
            rank: usize,
            file: usize,
            direction: i8,
            start_rank: usize,
        ) {
            if rank != start_rank as usize {
                return;
            }

            let from_sq = (rank * 8 + file) as u8;
            match board.get_piece((rank as i8 + direction) as usize, file) {
                None => match board.get_piece((rank as i8 + 2 * direction) as usize, file) {
                    None => {
                        let to_sq: u8 = ((rank as i8 + (2 * direction)) as usize * 8 + file) as u8;
                        pseudo_legal_moves.push(ChessMove::new(from_sq, to_sq, None));
                    }
                    _ => return,
                },
                _ => return,
            }
        }

        fn generate_pawn_forward_moves(
            board: &Board,
            pseudo_legal_moves: &mut Vec<ChessMove>,
            rank: usize,
            file: usize,
            direction: i8,
            promotion_trigger_rank: usize,
        ) {
            let new_rank = ((rank as i8) + direction) as usize;
            let new_file = file;

            match board.get_piece(new_rank, new_file) {
                None => {
                    let from_sq: u8 = (rank * 8 + file) as u8;
                    let to_sq: u8 = ((new_rank) * 8 + file) as u8;

                    if rank != (promotion_trigger_rank) as usize {
                        pseudo_legal_moves.push(ChessMove::new(from_sq, to_sq, None));
                        return;
                    }

                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq,
                        Some(Flag::Promotion(PieceType::Bishop)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq,
                        Some(Flag::Promotion(PieceType::Knight)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq,
                        Some(Flag::Promotion(PieceType::Queen)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq,
                        Some(Flag::Promotion(PieceType::Rook)),
                    ));
                }
                _ => return,
            }
        }

        fn generate_pawn_capture_moves(
            board: &Board,
            pseudo_legal_moves: &mut Vec<ChessMove>,
            rank: usize,
            file: usize,
            direction: i8,
            promotion_trigger_rank: usize,
            side: Color,
        ) {
            let from_sq = (rank * 8 + file) as u8;
            let new_rank = (rank as i8 + direction) as i8;

            let new_file_left = (file as i8) - 1;
            if Position::check_sq_in_board(new_rank, new_file_left)
                && Position::check_enemy_piece(board, side, new_rank, new_file_left)
            {
                let to_sq_left: u8 = (new_rank * 8 + new_file_left) as u8;
                if rank == promotion_trigger_rank as usize {
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_left,
                        Some(Flag::Promotion(PieceType::Bishop)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_left,
                        Some(Flag::Promotion(PieceType::Knight)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_left,
                        Some(Flag::Promotion(PieceType::Queen)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_left,
                        Some(Flag::Promotion(PieceType::Rook)),
                    ));
                } else {
                    pseudo_legal_moves.push(ChessMove::new(from_sq, to_sq_left, None));
                }
            }

            let new_file_right = (file as i8) + 1;
            if Position::check_sq_in_board(new_rank, new_file_right)
                && Position::check_enemy_piece(board, side, new_rank, new_file_right)
            {
                let to_sq_right: u8 = (new_rank * 8 + new_file_right) as u8;
                if rank == promotion_trigger_rank as usize {
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_right,
                        Some(Flag::Promotion(PieceType::Bishop)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_right,
                        Some(Flag::Promotion(PieceType::Knight)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_right,
                        Some(Flag::Promotion(PieceType::Queen)),
                    ));
                    pseudo_legal_moves.push(ChessMove::new(
                        from_sq,
                        to_sq_right,
                        Some(Flag::Promotion(PieceType::Rook)),
                    ));
                } else {
                    pseudo_legal_moves.push(ChessMove::new(from_sq, to_sq_right, None));
                }
            }
        }

        let (direction, start_rank, last_rank, promotion_trigger_rank) = match side {
            Color::White => (1i8, 1usize, 7usize, 6usize),
            Color::Black => (-1i8, 6usize, 0usize, 1usize),
        };

        for sq in 0..64 {
            let rank = sq / 8 as usize;
            let file = sq % 8 as usize;

            match board.get_piece(rank, file) {
                None => continue,
                Some(piece) => {
                    if piece.piece_type != PieceType::Pawn || piece.color != side {
                        continue;
                    }

                    if rank == last_rank {
                        panic!("There shouldn't be a pawn here!!!");
                    }

                    // generate starting move
                    generate_pawn_starting_move(
                        board,
                        pseudo_legal_moves,
                        rank,
                        file,
                        direction,
                        start_rank,
                    );
                    // generate forward moves
                    generate_pawn_forward_moves(
                        board,
                        pseudo_legal_moves,
                        rank,
                        file,
                        direction,
                        promotion_trigger_rank,
                    );
                    // generate capture moves
                    generate_pawn_capture_moves(
                        board,
                        pseudo_legal_moves,
                        rank,
                        file,
                        direction,
                        promotion_trigger_rank,
                        side,
                    );
                }
            }
        }
    }

    fn check_sq_in_board(rank: i8, file: i8) -> bool {
        (0 <= rank && rank < 8) && (0 <= file && file < 8)
    }

    fn check_ally_piece(board: &Board, side: Color, rank: i8, file: i8) -> bool {
        match board.get_piece(rank as usize, file as usize) {
            Some(piece) if piece.color == side => true,
            _ => false,
        }
    }

    fn check_enemy_piece(board: &Board, side: Color, rank: i8, file: i8) -> bool {
        match board.get_piece(rank as usize, file as usize) {
            Some(piece) if piece.color != side => true,
            _ => false,
        }
    }
}


#[cfg(test)] mod tests {
    use super::*;

    #[test]
    fn knight_alone_on_a1_generating_two_moves() {
        let position = Position::new_with_fen("8/8/8/8/8/8/8/N7 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 2, "knight on a1 should have exactly 2 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(0, 17, None),
            ChessMove::new(0, 10, None)
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }
    
    #[test]
    fn knight_blocked_by_own_pieces() {
        let position = Position::new_with_fen("8/8/8/8/3N4/8/2P1P3/8 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 10, "knight on d4 should have exactly 10 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 3), ChessMove::rank_and_file_to_square(5, 2), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 3), ChessMove::rank_and_file_to_square(5, 4), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 3), ChessMove::rank_and_file_to_square(4, 1), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 3), ChessMove::rank_and_file_to_square(4, 5), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 3), ChessMove::rank_and_file_to_square(2, 1), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 3), ChessMove::rank_and_file_to_square(2, 5), None),

            ChessMove::new(ChessMove::rank_and_file_to_square(1, 2), ChessMove::rank_and_file_to_square(2, 2), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 2), ChessMove::rank_and_file_to_square(3, 2), None),
            
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 4), ChessMove::rank_and_file_to_square(2, 4), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 4), ChessMove::rank_and_file_to_square(3, 4), None),
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }

    #[test]
    fn king_wrap_around_edge_count() {
        let position = Position::new_with_fen("8/8/8/8/8/8/8/K7 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 3, "knight on a1 should have exactly 3 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(ChessMove::rank_and_file_to_square(0, 0), ChessMove::rank_and_file_to_square(1, 0), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(0, 0), ChessMove::rank_and_file_to_square(0, 1), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(0, 0), ChessMove::rank_and_file_to_square(1, 1), None),
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }
    
    #[test]
    fn king_blocked_by_own_piece() {
        let position = Position::new_with_fen("8/8/8/8/4K3/4P3/8/8 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 7, "king on a4 should have exactly 7 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(4, 3), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(3, 3), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(2, 3), None),
            
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(4, 5), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(3, 5), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(2, 5), None),
            
            ChessMove::new(ChessMove::rank_and_file_to_square(3, 4), ChessMove::rank_and_file_to_square(4, 4), None),
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }

    #[test]
    fn pawn_double_push_blocked() {
        let position = Position::new_with_fen("8/8/8/8/4n3/8/4P3/8 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 1, "pawn on e2 should have exactly 1 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 4), ChessMove::rank_and_file_to_square(2, 4), None),
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }
    
    #[test]
    fn pawn_diagonal_capture() {
        let position = Position::new_with_fen("8/8/8/8/8/3p4/4P3/8 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 3, "pawn on e2 should have exactly 3 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 4), ChessMove::rank_and_file_to_square(2, 4), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 4), ChessMove::rank_and_file_to_square(3, 4), None),
            ChessMove::new(ChessMove::rank_and_file_to_square(1, 4), ChessMove::rank_and_file_to_square(2, 3), None),
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }
    
    #[test]
    fn pawn_promotion() {
        let position = Position::new_with_fen("8/4P3/8/8/8/8/8/8 w - - 0 1");

        let moves: Vec<ChessMove> = position.generate_pseudo_legal_moves();

        assert_eq!(moves.len(), 4, "pawn on e2 should have exactly 4 moves, got {:#?}", moves);

        let expected_moves: Vec<ChessMove> = vec![
            ChessMove::new(ChessMove::rank_and_file_to_square(6, 4), ChessMove::rank_and_file_to_square(7, 4), Some(Flag::Promotion((PieceType::Rook)))),
            ChessMove::new(ChessMove::rank_and_file_to_square(6, 4), ChessMove::rank_and_file_to_square(7, 4), Some(Flag::Promotion((PieceType::Queen)))),
            ChessMove::new(ChessMove::rank_and_file_to_square(6, 4), ChessMove::rank_and_file_to_square(7, 4), Some(Flag::Promotion((PieceType::Bishop)))),
            ChessMove::new(ChessMove::rank_and_file_to_square(6, 4), ChessMove::rank_and_file_to_square(7, 4), Some(Flag::Promotion((PieceType::Knight)))),
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move), "Missing move: {:#?}", expected_move);
        }
    }

}