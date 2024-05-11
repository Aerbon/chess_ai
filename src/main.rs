use chess::{Board, ChessMove, MoveGen};
use chess_ai::{choose_best, VERY_BIG_NUM};
use clearscreen::ClearScreen;
use colored::Colorize;
use text_io::read;

use std::io::{stdout, Write};

fn main() {
    println!("Hello, world!");

    print!("AI difficulty level? ");
    let smortness: u8 = read!("{}\n");

    let mut players = (Player::Player, Player::Bot);
    print!("Player 1 type: ");
    let user_in: String = read!("{}\n");
    match user_in.as_str() {
        "bot" => players.0 = Player::Bot,
        _ => players.0 = Player::Player,
    }
    print!("Player 2 type: ");
    let user_in: String = read!("{}\n");
    match user_in.as_str() {
        "bot" => players.1 = Player::Bot,
        _ => players.1 = Player::Player,
    }
    drop(user_in);

    let mut board: Board = Board::default();
    let mut history: Vec<ChessMove> = vec![];

    loop {
        print_board(board);
        // println!("Last moves: {}", history.last());
        match board.status() {
            chess::BoardStatus::Ongoing => (),
            chess::BoardStatus::Stalemate => {
                println!("{}", "STALEMATE.".bold().yellow());
                return;
            }
            chess::BoardStatus::Checkmate => {
                match board.side_to_move() {
                    chess::Color::White => println!("Black wins!"),
                    chess::Color::Black => println!("White wins!"),
                }
                return;
            }
        }
        let moves = MoveGen::new_legal(&board);
        let selected_move: String;
        match board.side_to_move() {
            chess::Color::White => {
                println!("White to move.");
                match players.0 {
                    Player::Player => {
                        print!("?> ");
                        selected_move = read!("{}\n");
                    }
                    Player::Bot => {
                        let (m, _) = choose_best(&board, smortness, VERY_BIG_NUM);
                        selected_move = format!("{}", m);
                    }
                }
            }
            chess::Color::Black => {
                println!("Black to move.");
                match players.1 {
                    Player::Player => {
                        print!("?> ");
                        selected_move = read!("{}\n");
                    }
                    Player::Bot => {
                        let (m, _) = choose_best(&board, smortness, -VERY_BIG_NUM);
                        selected_move = format!("{}", m);
                    }
                }
            }
        }
        for m in moves {
            if selected_move == format!("{}", m) {
                // println!("accepted.");
                board = board.make_move_new(m);
                history.push(m);
                break;
            }
        }
    }
}

fn print_board(board: Board) {
    let mut lock = stdout().lock();
    const PALETTE: [(u8, u8, u8); 2] = [(128, 96, 64), (64, 32, 16)];
    let mut row = 0;
    let mut col = 0;
    ClearScreen::default().clear().expect("morning rescue");
    write!(lock, "8").unwrap();
    let fen = format!("{}", board);
    for p in fen.chars() {
        match p {
            ' ' => {
                write!(lock, "\n a b c d e f g h\n").unwrap();
                return;
            }
            '/' => {
                row = 0;
                col += 1;
                write!(lock, "\n{}", 8 - col).unwrap();
            }
            '1'..='8' => {
                for _ in 1..=match p.to_digit(10) {
                    Some(a) => a,
                    _ => 0,
                } {
                    let c = PALETTE[(row + col) % 2];
                    write!(lock, "{}", "  ".to_string().on_truecolor(c.0, c.1, c.2)).unwrap();
                    row += 1;
                }
            }
            _ => {
                let c = PALETTE[(row + col) % 2];
                write!(
                    lock,
                    "{}",
                    (p.to_ascii_uppercase().to_string() + " ")
                        .bold()
                        .truecolor(color(p).0, color(p).1, color(p).2)
                        .on_truecolor(c.0, c.1, c.2)
                )
                .unwrap();
                row += 1;
            }
        }
    }
}

fn color(p: char) -> (u8, u8, u8) {
    match p.is_uppercase() {
        true => (255, 255, 255),
        false => (0, 0, 0),
    }
}

enum Player {
    Player,
    Bot,
}
