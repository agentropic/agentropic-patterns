//! Hierarchical organization patterns

/// Task delegation
pub mod delegation;
/// Organizational levels
pub mod level;
/// Hierarchy structure
pub mod structure;

pub use delegation::Delegation;
pub use level::{Level, LevelType};
pub use structure::Hierarchy;
