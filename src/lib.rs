//! Organizational patterns and architectures for multi-agent systems.

//#![warn(missing_docs)]
#![allow(missing_docs)]

// patterns
pub mod blackboard;
pub mod coalition;
pub mod error;
pub mod federation;
pub mod hierarchy;
pub mod holarchy;
pub mod market;
pub mod prelude;
pub mod swarm;
pub mod team;

// Re-exports
pub use error::PatternError;
