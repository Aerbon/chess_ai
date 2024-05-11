use colored::Colorize;

#[derive(Copy, Clone)]
enum Piece {
    None,
    PawnW,
    PawnB,
    KnightW,
    KnightB,
    BishopW,
    BishopB,
    RookW,
    RookB,
    QueenW,
    QueenB,
    KingW,
    KingB,
}

const COLOR_SCHEME: [(u8, u8, u8); 4] = [(255, 255, 255), (128, 96, 64), (0, 0, 0), (64, 32, 16)];

pub type Board = [[Piece; 8]; 8];

#[derive(Copy, Clone)]
pub struct GameState {
    board: Board,
    turn_player: Player,
    turns_elapsed: u32,
}

#[derive(Copy, Clone)]
enum Player {
    White,
    Black,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [
                [
                    Piece::RookB,
                    Piece::KnightB,
                    Piece::BishopB,
                    Piece::QueenB,
                    Piece::KingB,
                    Piece::BishopB,
                    Piece::KnightB,
                    Piece::RookB,
                ],
                [Piece::PawnB; 8],
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                [Piece::PawnW; 8],
                [
                    Piece::RookW,
                    Piece::KnightW,
                    Piece::BishopW,
                    Piece::QueenW,
                    Piece::KingW,
                    Piece::BishopW,
                    Piece::KnightW,
                    Piece::RookW,
                ],
            ],
            turn_player: Player::White,
            turns_elapsed: 0,
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self::None
    }
}

impl Piece {
    const fn value(self) -> i32 {
        match self {
            Piece::None => 0,
            Piece::PawnW => 1,
            Piece::PawnB => -1,
            Piece::KnightW => 3,
            Piece::KnightB => -3,
            Piece::BishopW => 3,
            Piece::BishopB => -3,
            Piece::RookW => 4,
            Piece::RookB => -4,
            Piece::QueenW => 9,
            Piece::QueenB => -9,
            Piece::KingW => 0,
            Piece::KingB => 0,
        }
    }
    const fn allegiance(self) -> i32 {
        match self {
            Piece::None => 0,
            Piece::PawnW => 1,
            Piece::PawnB => -1,
            Piece::KnightW => 1,
            Piece::KnightB => -1,
            Piece::BishopW => 1,
            Piece::BishopB => -1,
            Piece::RookW => 1,
            Piece::RookB => -1,
            Piece::QueenW => 1,
            Piece::QueenB => -1,
            Piece::KingW => 1,
            Piece::KingB => -1,
        }
    }
    const fn team(self) -> u8 {
        match self {
            Piece::None => 0,
            Piece::PawnW => 1,
            Piece::PawnB => 2,
            Piece::KnightW => 1,
            Piece::KnightB => 2,
            Piece::BishopW => 1,
            Piece::BishopB => 2,
            Piece::RookW => 1,
            Piece::RookB => 2,
            Piece::QueenW => 1,
            Piece::QueenB => 2,
            Piece::KingW => 1,
            Piece::KingB => 2,
        }
    }
    const fn character(self) -> char {
        match self {
            Piece::None => ' ',
            Piece::PawnW => 'P',
            Piece::PawnB => 'p',
            Piece::KnightW => 'N',
            Piece::KnightB => 'n',
            Piece::BishopW => 'B',
            Piece::BishopB => 'b',
            Piece::RookW => 'R',
            Piece::RookB => 'r',
            Piece::QueenW => 'Q',
            Piece::QueenB => 'q',
            Piece::KingW => 'K',
            Piece::KingB => 'k',
        }
    }
}

impl Player {
    fn next(self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
    fn side(self) -> i32 {
        match self {
            Player::White => 1,
            Player::Black => -1,
        }
    }
}

pub fn change_board(b: &mut Board, c: Vec<(usize, Piece)>) {
    for (position, piece) in c {
        b[position / b[0].len()][position % b[0].len()] = piece;
    }
}

impl GameState {
    pub fn print_board(self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                let c1: (u8, u8, u8) = COLOR_SCHEME[(1 - self.board[y][x].allegiance()) as usize];
                let c2 = COLOR_SCHEME[((x + y) % 2) * 2 + 1];
                print!(
                    "{}",
                    self.board[y][x]
                        .character()
                        .to_string()
                        .truecolor(c1.0, c1.1, c1.2)
                        .on_truecolor(c2.0, c2.1, c2.2)
                        .bold()
                );
            }
            print!("\n");
        }
    }
    pub fn set_piece(mut self, change: (usize, usize, Piece)) -> Self {
        self.board[change.1][change.0] = change.2;
        self
    }
    pub fn set_pieces(mut self, changes: Vec<(usize, usize, Piece)>) -> Self {
        for c in changes {
            self.board[c.1][c.0] = c.2;
        }
        self
    }
}

mod notation;

mod gameplay;
