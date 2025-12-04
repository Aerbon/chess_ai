use burn::{data::dataloader::batcher::Batcher, prelude::*};
use chess::{Board, Square};

#[derive(Clone, Default)]
pub struct ChessBatcher {}

#[derive(Clone, Debug)]
pub struct ChessBatch<B: Backend> {
    pub pieces: Tensor<B, 4>,
    pub properties: Tensor<B, 2>,
    pub targets: Tensor<B, 2>
}

impl<B: Backend> Batcher<B, (Board, f32), ChessBatch<B>> for ChessBatcher {
    fn batch(&self, items: Vec<(Board, f32)>, device: &B::Device) -> ChessBatch<B> {
        let mut pies = vec![];
        let mut pros = vec![];
        let mut tars = vec![];
        items.into_iter().for_each(|(bor, tar)|{
            tars.push(Tensor::from_data([[tar]], device));
            let (pie, pro) = board_to_tensors(bor, device);
            pies.push(pie.reshape([1_i32,0,0,-1]));
            pros.push(pro.reshape([1_i32,-1]));
        });
        ChessBatch {
            pieces: Tensor::cat(pies,0),
            properties: Tensor::cat(pros,0),
            targets: Tensor::cat(tars,0),
        }
    }
}

fn board_to_tensors<B: Backend>(bor: Board, device: &B::Device) -> (Tensor<B, 3>, Tensor<B, 1>) {
    let mut data: [[[f32;chess::NUM_PIECES];chess::NUM_FILES];chess::NUM_RANKS] = Default::default();
    for i in 0..64 {
        assert!(i < 64); // Just in case
        let sq = unsafe {Square::new(i)};
        let c = match bor.color_on(sq) {
            Some(chess::Color::White) => 1.0,
            Some(chess::Color::Black) => -1.0,
            Option::None => 0.0,
        };
        match bor.piece_on(sq) {
            Some(piece) => data[i as usize / chess::NUM_FILES][i as usize % chess::NUM_RANKS][piece as usize] = c,
            Option::None => (),
        }
    }
    (
        Tensor::from_data(data, device),
        Tensor::from_data([0.0_f32,0.0,0.0,0.0], device)
    )
}