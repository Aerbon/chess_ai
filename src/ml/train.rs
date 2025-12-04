use burn::data::dataloader::DataLoaderBuilder;
use burn::data::dataset::InMemDataset;
use burn::record::CompactRecorder;
use burn::train::metric::{AccuracyMetric, LossMetric};
use burn::train::{LearnerBuilder, LearningStrategy, RegressionOutput, TrainOutput, TrainStep, ValidStep};
use burn::{optim, tensor::backend::AutodiffBackend};
use burn::prelude::*;
use chess::Board;

use crate::ml::data::ChessBatch;
use crate::ml::{self, data::ChessBatcher};

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub optimizer: optim::AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

pub fn train_model<B: AutodiffBackend>(artifact_dir: &str, model: ml::ChessModel<B>, dataset: Vec<(Board, f32)>, dataset_val: Vec<(Board, f32)>, config: TrainingConfig, device: B::Device) {
    B::seed(&device, config.seed);
    let batcher = ChessBatcher::default();
    let dataset = InMemDataset::new(dataset);
    let dataset_val = InMemDataset::new(dataset_val);
    let dataloader_tra = DataLoaderBuilder::new(batcher.clone())
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(dataset);

    let dataloader_val = DataLoaderBuilder::new(batcher)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(dataset_val);
    
    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .learning_strategy(LearningStrategy::SingleDevice(device.clone()))
        .num_epochs(10)
        .summary()
        .build(model, config.optimizer.init(), config.learning_rate);
    let result = learner.fit(dataloader_tra, dataloader_val);
}

impl<B: AutodiffBackend> TrainStep<ChessBatch<B>, RegressionOutput<B>> for ml::ChessModel<B> {
    fn step(&self, item: ChessBatch<B>) -> burn::train::TrainOutput<RegressionOutput<B>> {
        let item = self.forward_regression(item);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<ChessBatch<B>, RegressionOutput<B>> for ml::ChessModel<B> {
    fn step(&self, item: ChessBatch<B>) -> RegressionOutput<B> {
        self.forward_regression(item)
    }
}