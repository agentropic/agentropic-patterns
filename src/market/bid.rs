use agentropic_core::AgentId;
use serde::{Deserialize, Serialize};

/// Bid in an auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    bidder: AgentId,
    amount: f64,
    resource: String,
}

impl Bid {
    /// Create a new bid
    pub fn new(bidder: AgentId, amount: f64, resource: impl Into<String>) -> Self {
        Self {
            bidder,
            amount,
            resource: resource.into(),
        }
    }

    /// Get bidder
    pub fn bidder(&self) -> &AgentId {
        &self.bidder
    }

    /// Get amount
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get resource
    pub fn resource(&self) -> &str {
        &self.resource
    }
}
