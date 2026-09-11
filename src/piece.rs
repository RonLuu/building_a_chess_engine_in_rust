use std::fmt;


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Color {
    Black,
    White
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White"),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "Pawn"),
            PieceType::Knight => write!(f, "Knight"),
            PieceType::Bishop => write!(f, "Bishop"),
            PieceType::Rook => write!(f, "Rook"),
            PieceType::Queen => write!(f, "Queen"),
            PieceType::King => write!(f, "King"),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}

impl Piece {
    fn get_piece_from_char(c: char) -> Piece {
        match c {
            'r' => Piece {piece_type: PieceType::Rook, color: Color::Black},
            'R' => Piece {piece_type: PieceType::Rook, color: Color::White},
            
            'b' => Piece {piece_type: PieceType::Bishop, color: Color::Black},
            'B' => Piece {piece_type: PieceType::Bishop, color: Color::White},
            
            'n' => Piece {piece_type: PieceType::Knight, color: Color::Black},
            'N' => Piece {piece_type: PieceType::Knight, color: Color::White},
            
            'p' => Piece {piece_type: PieceType::Pawn, color: Color::Black},
            'P' => Piece {piece_type: PieceType::Pawn, color: Color::White},
            
            'q' => Piece {piece_type: PieceType::Queen, color: Color::Black},
            'Q' => Piece {piece_type: PieceType::Queen, color: Color::White},
            
            'k' => Piece {piece_type: PieceType::King, color: Color::Black},
            'K' => Piece {piece_type: PieceType::King, color: Color::White},

            _ => panic!("Illegal FEN: Wrong character piece")
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.color, self.piece_type) {
            (Color::White, PieceType::Pawn) => write!(f, "P"),
            (Color::White, PieceType::Bishop) => write!(f, "B"),
            (Color::White, PieceType::Knight) => write!(f, "N"),
            (Color::White, PieceType::Rook) => write!(f, "R"),
            (Color::White, PieceType::Queen) => write!(f, "Q"),
            (Color::White, PieceType::King) => write!(f, "K"),
            (Color::Black, PieceType::Pawn) => write!(f, "p"),
            (Color::Black, PieceType::Bishop) => write!(f, "b"),
            (Color::Black, PieceType::Knight) => write!(f, "n"),
            (Color::Black, PieceType::Rook) => write!(f, "r"),
            (Color::Black, PieceType::Queen) => write!(f, "q"),
            (Color::Black, PieceType::King) => write!(f, "k"),
        }               
    }
}