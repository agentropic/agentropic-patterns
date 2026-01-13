use serde::{Deserialize, Serialize};

/// Type of coalition strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyType {
    /// Maximize utility
    MaximizeUtility,
    /// Minimize cost
    MinimizeCost,
    /// Balance resources
    BalanceResources,
    /// Maximize coverage
    MaximizeCoverage,
}

/// Coalition strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    strategy_type: StrategyType,
    parameters: Vec<(String, f64)>,
}

impl Strategy {
    /// Create a new strategy
    pub fn new(strategy_type: StrategyType) -> Self {
        Self {
            strategy_type,
            parameters: Vec::new(),
        }
    }

    /// Add parameter
    pub fn with_parameter(mut self, name: impl Into<String>, value: f64) -> Self {
        self.parameters.push((name.into(), value));
        self
    }

    /// Get strategy type
    pub fn strategy_type(&self) -> StrategyType {
        self.strategy_type
    }

    /// Get parameters
    pub fn parameters(&self) -> &[(String, f64)] {
        &self.parameters
    }
}
