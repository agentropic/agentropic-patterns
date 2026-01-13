//! Hierarchical organization patterns

/// Task delegation
pub mod delegation;
/// Hierarchy structure
pub mod hierarchy;
/// Organizational levels
pub mod level;

pub use delegation::Delegation;
pub use hierarchy::Hierarchy;
pub use level::{Level, LevelType};
