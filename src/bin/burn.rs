#![recursion_limit = "256"]

// use burn::tensor::Tensor;
use burn::backend::{Autodiff, Wgpu};
use burn::optim::AdamConfig;
use burn::prelude::*;
use burn::record::{CompactRecorder, Recorder};
use burn::module::AutodiffModule;
use chess::Board;
use chess_ai::ml::infer::ChessPlayerAgent;
use chess_ai::{
    game::ChessGame,
    ml::{self, ChessModel},
    players::{SimpleMinMax, Simple},
};

// use rayon::prelude::*;

use clearscreen::ClearScreen;
use colored::Colorize;
// use rand::Rng;

use std::io::{stdout, Write};

const SAMPLE_SIZE: usize = 1024;
const EPOCH_COUNT: usize = 10;
const MATCH_TYPES: usize = 1;

fn main() {
    std::fs::create_dir_all("arcades").ok();
    let device = Default::default();
    let chess_model_config = ml::ChessModelConfig::new(1024, 128);
    let training_config = ml::train::TrainingConfig::new(AdamConfig::new());
    let mut chess_ai_model: ChessModel<Autodiff<Wgpu>> = match CompactRecorder::new().load("arcades/model".into(), &device) {
        Ok(rec) => chess_model_config.init(&device).load_record(rec),
        Err(_) => chess_model_config.init(&device)
    };
    loop {
        let mut dataset: Vec<(Board, Vec<f32>)> = vec!();
        let mut metrics: Vec<(std::time::Duration, std::time::Duration)> = Default::default(); 
        // let ds = (1..=64).into_par_iter().map(|x| x % 2 != 0).map(|x| match x {
        //     true => ChessGame::new(p1.clone(), p2.clone()).run(),
        //     false => ChessGame::new(p2.clone(), p1.clone()).run()
        // });
        println!("Playing matches...");
        for game in 0..SAMPLE_SIZE {
            let elapsed = std::time::Instant::now();
            let (hist, score) = match game % MATCH_TYPES {
                0 => ChessGame::new(
                    ChessPlayerAgent::new(chess_ai_model.clone(), 3),
                    ChessPlayerAgent::new(chess_ai_model.clone(), 3)
                ).run(),
                1 => ChessGame::new(
                    Simple{},
                    ChessPlayerAgent::new(chess_ai_model.clone(), 3)
                ).run(),
                2 => ChessGame::new(
                    ChessPlayerAgent::new(chess_ai_model.clone(), 3),
                    Simple{}
                ).run(),
                _ => ChessGame::new(
                    Simple{},
                    Simple{}
                ).run()
            };
            let elapsed = elapsed.elapsed();
            print_board(hist[hist.len() - 1]);
            println!("Match {}/{SAMPLE_SIZE} (Type {})", game + 1, game % MATCH_TYPES);
            println!("Result: {score:?}, duration: {elapsed:#?}");
            if metrics.len() <= game % MATCH_TYPES {
                metrics.push((elapsed, elapsed));
            } else {
                let (mn, mx) = metrics[game % MATCH_TYPES];
                if mn > elapsed {
                    metrics[game % MATCH_TYPES].0 = elapsed;
                } else if mx < elapsed {
                    metrics[game % MATCH_TYPES].1 = elapsed;
                }
            }
            let score = match score {
                chess::GameResult::WhiteCheckmates => 1.0,
                chess::GameResult::WhiteResigns => -0.95,
                chess::GameResult::BlackCheckmates => -1.0,
                chess::GameResult::BlackResigns => 0.95,
                chess::GameResult::Stalemate => 0.0,
                chess::GameResult::DrawAccepted => 0.0,
                chess::GameResult::DrawDeclared => 0.0,
            };
            let data: Vec<(chess::Board, f32)> = hist.into_iter().map(|b| (b, score)).collect();
            for board_result in data {
                let hash = board_result.0.get_hash();
                match dataset.iter().position(|(b, _)| b.get_hash() == hash) {
                    Option::None => {
                        dataset.push((board_result.0, vec!(board_result.1)));
                    },
                    Option::Some(i) => {
                        dataset[i].1.push(board_result.1);
                    }
                }
            }
        }
        println!("Metrics:");
        for i in 0..MATCH_TYPES {
            let (mn, mx) = metrics[i];
            println!("Type {i}: Min: {mn:#?}, Max: {mx:#?}",);
        }
        println!("Compiling data set...");
        let dataset: Vec<(Board, f32)> = dataset.into_iter().map(|(b, scores)| (b, scores.iter().sum::<f32>() / scores.len() as f32)).collect();
        // println!("Match over, score: {score}!");
        println!("Training...");
        for _epoch in 1..=EPOCH_COUNT {
            chess_ai_model = chess_ai_model.learn_from(&training_config, dataset.clone());
        }
        println!("Saving model...");
        chess_ai_model.clone().save_file("arcades/model", &CompactRecorder::new()).expect("File Could Not Be Saved.");
        println!("Exhibition match!");
        let (hist, score) = ChessGame::new(
            ChessPlayerAgent::new(chess_ai_model.valid().clone(), 2),
            SimpleMinMax::new(5)
        ).run();
        for board in hist {
            println!("\x07");
            print_board(board);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        println!("Result: {score:?}");
    }
}

fn print_board(board: chess::Board) {
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
