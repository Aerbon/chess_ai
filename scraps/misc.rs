pub const PIECES: [PieceData; PIECE_COUNT] = [
    PieceData {
        name: "Pawn",
        value: 1,
    },
    PieceData {
        name: "Knight",
        value: 3,
    },
    PieceData {
        name: "Bishop",
        value: 3,
    },
    PieceData {
        name: "Rook",
        value: 4,
    },
    PieceData {
        name: "Queen",
        value: 9,
    },
    PieceData {
        name: "King",
        value: 0,
    },
];

#[derive(Copy, Clone)]
enum Piece {
    White(u8),
    Black(u8),
    None,
}

pub struct PieceData {
    name: &'static str,
    value: u8,
}

pub const PIECE_COUNT: usize = 6;

struct Board {
    pieces: [u8; 64],
}

impl Board {
    pub fn sample() -> Piece {
        todo!()
    }
}
