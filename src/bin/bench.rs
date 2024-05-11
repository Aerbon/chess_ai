use chess::{Board, MoveGen};
use chess_ai::{choose_best, VERY_BIG_NUM};

fn main() {
    let board: Board = Default::default();
    for depth in 1..=8 {
        println!("benching level {}", depth);
        let moves = MoveGen::new_legal(&board);
        for m in moves {
            let (_, score) = choose_best(&board, depth - 1, VERY_BIG_NUM);
            println!("{} scores {}", m, score);
        }
    }
}
