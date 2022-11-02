pub mod challenge;
pub mod event;
pub mod game;
pub mod lichess;
pub mod stream;
pub mod utils;

pub type ResultReturn = Result<(), Box<dyn std::error::Error>>;
