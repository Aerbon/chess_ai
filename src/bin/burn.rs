// use burn::tensor::Tensor;
use burn::backend::Wgpu;
use burn::data::dataloader::batcher;
use burn::prelude::*;
use chess_ai::ml;

// Type alias for the backend to use.
type MyBackend = Wgpu;



fn main() {
    let device = Default::default();
    let chess_model_config = ml::ChessModelConfig::new(1024, 128);
    let mut chess_ai_model = chess_model_config.init(device);
    let batcher = ml::ChessBatcher::default();
    
}