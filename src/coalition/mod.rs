//! Coalition formation patterns

/// Coalition formation algorithms
pub mod formation;
/// Coalition strategy
pub mod strategy;
/// Coalition structure
pub mod structure;

pub use formation::{Formation, FormationType};
pub use strategy::{Strategy, StrategyType};
pub use structure::Coalition;
