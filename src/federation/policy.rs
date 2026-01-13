use serde::{Deserialize, Serialize};

/// Type of federation policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    /// Consensus-based decision
    Consensus,
    /// Majority vote
    MajorityVote,
    /// Weighted vote
    WeightedVote,
    /// Democratic
    Democratic,
}

/// Federation policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    name: String,
    policy_type: PolicyType,
    threshold: f64,
    rules: Vec<String>,
}

impl Policy {
    /// Create a new policy
    pub fn new(name: impl Into<String>, policy_type: PolicyType) -> Self {
        Self {
            name: name.into(),
            policy_type,
            threshold: 0.5,
            rules: Vec::new(),
        }
    }

    /// Set threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Add rule
    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rules.push(rule.into());
        self
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get policy type
    pub fn policy_type(&self) -> PolicyType {
        self.policy_type
    }

    /// Get threshold
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Get rules
    pub fn rules(&self) -> &[String] {
        &self.rules
    }
}
