//! Swarm intelligence patterns

/// Swarm behaviors
pub mod behavior;
/// Consensus mechanisms
pub mod consensus;
/// Flocking behavior
pub mod flocking;
/// Foraging behavior
pub mod foraging;
/// Swarm structure
pub mod structure;

pub use behavior::{Behavior, BehaviorType};
pub use consensus::Consensus;
pub use flocking::Flocking;
pub use foraging::Foraging;
pub use structure::Swarm;
