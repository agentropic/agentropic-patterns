//! Blackboard architecture patterns

/// Knowledge source
pub mod knowledge_source;
/// Blackboard structure
pub mod structure;

pub use knowledge_source::{KnowledgeSource, KnowledgeSourceType};
pub use structure::Blackboard;
