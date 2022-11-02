pub mod api;
pub mod challenge;
pub mod event;
pub mod game;
pub mod utils;

pub(crate) type ResultReturn = Result<(), Box<dyn std::error::Error>>;
