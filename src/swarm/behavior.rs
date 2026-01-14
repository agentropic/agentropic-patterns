use serde::{Deserialize, Serialize};

/// Type of swarm behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorType {
    /// Flocking behavior
    Flocking,
    /// Foraging behavior
    Foraging,
    /// Exploration behavior
    Exploration,
    /// Aggregation behavior
    Aggregation,
}

/// Swarm behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    behavior_type: BehaviorType,
    parameters: Vec<(String, f64)>,
}

impl Behavior {
    /// Create a new behavior
    pub fn new(behavior_type: BehaviorType) -> Self {
        Self {
            behavior_type,
            parameters: Vec::new(),
        }
    }

    /// Add parameter
    pub fn with_parameter(mut self, name: impl Into<String>, value: f64) -> Self {
        self.parameters.push((name.into(), value));
        self
    }

    /// Get behavior type
    pub fn behavior_type(&self) -> BehaviorType {
        self.behavior_type
    }

    /// Get parameters
    pub fn parameters(&self) -> &[(String, f64)] {
        &self.parameters
    }
}
