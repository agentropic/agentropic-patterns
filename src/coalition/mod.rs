//! Coalition formation patterns

/// Coalition structure
pub mod coalition;
/// Coalition formation algorithms
pub mod formation;
/// Coalition strategy
pub mod strategy;

pub use coalition::Coalition;
pub use formation::{Formation, FormationType};
pub use strategy::{Strategy, StrategyType};
