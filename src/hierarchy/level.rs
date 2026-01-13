use serde::{Deserialize, Serialize};

/// Type of organizational level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LevelType {
    /// Strategic level
    Strategic,
    /// Tactical level
    Tactical,
    /// Operational level
    Operational,
}

/// Organizational level in hierarchy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    name: String,
    level_type: LevelType,
    rank: u32,
}

impl Level {
    /// Create a new level
    pub fn new(name: impl Into<String>, level_type: LevelType, rank: u32) -> Self {
        Self {
            name: name.into(),
            level_type,
            rank,
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the level type
    pub fn level_type(&self) -> LevelType {
        self.level_type
    }

    /// Get the rank
    pub fn rank(&self) -> u32 {
        self.rank
    }

    /// Check if this level is above another
    pub fn is_above(&self, other: &Level) -> bool {
        self.rank > other.rank
    }
}
