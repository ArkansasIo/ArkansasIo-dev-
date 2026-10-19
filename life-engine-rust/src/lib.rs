pub mod brain;
pub mod config;
pub mod ecology;
pub mod evolution;
pub mod genome;
pub mod neural;
pub mod organism;
pub mod physics;
pub mod replay;
pub mod stats;
pub mod unified;
pub mod wasm;
pub mod world;

pub use replay::Replay;
pub use stats::SimulationStats;
pub use unified::{Agent, LifeEngine};
