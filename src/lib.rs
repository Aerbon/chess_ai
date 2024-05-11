use std::sync::atomic::AtomicI32;

use chess::{Board, ChessMove, Color, MoveGen};
// use rayon::prelude::*;

pub const BIG_NUM: i32 = 0xffffff;
pub const VERY_BIG_NUM: i32 = BIG_NUM * 0x10;
pub const SMALLER_NUM: i32 = BIG_NUM / 0x10;

pub fn choose_best(board: &Board, l: u8, discard: i32) -> (ChessMove, i32) {
    let goal = match board.side_to_move() {
        Color::White => 1,
        Color::Black => -1,
    };
    match board.status() {
        chess::BoardStatus::Stalemate => return (ChessMove::default(), 0),
        chess::BoardStatus::Checkmate => return (ChessMove::default(), -BIG_NUM * goal),
        chess::BoardStatus::Ongoing => (),
    }
    let mut consider = MoveGen::new_legal(&board);
    let mut best: (ChessMove, i32);
    let mut d = -VERY_BIG_NUM * goal;
    if consider.len() == 0 {
        let m = consider.next().unwrap();
        let (_, s) = choose_best(&board.make_move_new(m), l, d);
        return (m, s);
    }
    match l {
        0 => {
            let m = consider.next().unwrap();
            let s = score(&board.make_move_new(m));
            best = (m, s);
            if (s - discard) * goal >= 0 {
                return best;
            }
            for m in consider {
                let s = score(&board.make_move_new(m));
                if (s - discard) * goal >= 0 {
                    return best;
                }
                match (s - best.1) * goal {
                    ..=0 => (),
                    _ => best = (m, s),
                }
            }
        }
        _ => {
            let m = consider.next().unwrap();
            let b = board.make_move_new(m);
            let (_, mut s) = choose_best(&b, l - 1, d);
            if s > SMALLER_NUM {
                s -= 1;
            }
            best = (m, s);
            if (s - discard) * goal >= 0 {
                return best;
            }
            if (s - d) * goal >= 0 {
                d = s;
            }
            for m in consider {
                let b = board.make_move_new(m);
                let (_, mut s) = choose_best(&b, l - 1, d);
                if s > SMALLER_NUM {
                    s -= 1;
                }
                if (s - discard) * goal >= 0 {
                    return best;
                }
                match (s - best.1) * goal {
                    ..=0 => (),
                    _ => best = (m, s),
                }
            }
            best.1 -= 1;
        }
    }
    return best;
}

pub fn choose_best_v2(board: &Board, l: u8) {
    type QItem<'a> = (ChessMove, Board, u8, &'a AtomicI32);
    let team = match board.side_to_move() {
        Color::White => 1,
        Color::Black => -1,
    };
    let d = AtomicI32::new(VERY_BIG_NUM * -team);
    let mut queue: Vec<QItem> = vec![];
    let movelist = MoveGen::new_legal(&board);
    for m in movelist {
        queue.push((m, *board, l - 1, &d));
    }
    while queue.len() > 0 {
        let item = queue.pop();
    }
    todo!();
}

fn score(board: &Board) -> i32 {
    match board.status() {
        chess::BoardStatus::Ongoing => board.combined().fold(0, |acc, x| {
            acc + match board.piece_on(x) {
                Some(p) => match p {
                    chess::Piece::Pawn => 100,
                    chess::Piece::Knight => 300,
                    chess::Piece::Bishop => 300,
                    chess::Piece::Rook => 500,
                    chess::Piece::Queen => 900,
                    chess::Piece::King => 0,
                },
                None => 0,
            } * match board.color_on(x) {
                Some(c) => match c {
                    Color::White => 1,
                    Color::Black => -1,
                },
                None => 0,
            }
        }),
        chess::BoardStatus::Stalemate => 0,
        chess::BoardStatus::Checkmate => match board.side_to_move() {
            chess::Color::White => -BIG_NUM,
            chess::Color::Black => BIG_NUM,
        },
    }
}
