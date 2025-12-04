use burn::{data::dataloader::batcher::Batcher, prelude::*};
use burn::train::RegressionOutput;
use chess::{Board, Square};

pub mod train;
pub mod data;

#[derive(Module,Debug)]
pub struct ChessModel<B: Backend> {
    // conv1: nn::conv::Conv2d<B>,
    // conv2: nn::conv::Conv2d<B>,
    full1: nn::Linear<B>,
    full2: nn::Linear<B>,
    out: nn::Linear<B>,
    dropout: nn::Dropout,
}

#[derive(Config,Debug)]
pub struct ChessModelConfig {
    hidden1: usize,
    hidden2: usize,
    #[config(default = "0.3")]
    dropout: f64,
}

impl<B: Backend> ChessModel<B> {
    /// # Shapes
    ///   - Board [batch_size, height, width, pieces]
    ///   - Output [batch_size, 1]
    pub fn forward(&self, board: Tensor<B, 4>, props: Tensor<B, 2>) -> Tensor<B, 2> {
        let [batch_size , _height,_width,_pieces] = board.dims();
        let x = board.reshape([batch_size as i32,-1_i32]);
        let x = Tensor::cat(vec![x, props], 1);
        let x = self.dropout.forward(x);
        let x = self.full1.forward(x);
        let x = self.full2.forward(x);
        let x = self.dropout.forward(x);
        let x = self.out.forward(x);
        x
    }

    pub fn forward_regression(&self, batch: data::ChessBatch<B>) -> RegressionOutput<B> {
        let output = self.forward(batch.pieces, batch.properties);
        let loss = nn::loss::MseLoss::new()
            // .init(&output.device())
            .forward(output.clone(),batch.targets.clone(),nn::loss::Reduction::Sum);
        RegressionOutput {
            loss: loss,
            output: output,
            targets: batch.targets,
        }
    }
}

impl ChessModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> ChessModel<B> {
        ChessModel {
            // conv1: nn::conv::Conv2dConfig::new([9,8], [3,3]).with_padding(nn::PaddingConfig2d::Same).init(device),
            // conv2: nn::conv::Conv2dConfig::new([8,4], [7,7]).with_padding(nn::PaddingConfig2d::Same).init(device),
            full1: nn::LinearConfig::new(64*9 + 4, self.hidden1).init(device),
            full2: nn::LinearConfig::new(self.hidden1, self.hidden2).init(device),
            out: nn::LinearConfig::new(self.hidden2, 1).init(device),
            dropout: nn::DropoutConfig::new(self.dropout).init(),
        }
    }
}

