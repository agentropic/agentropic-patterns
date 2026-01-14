use thiserror::Error;

// Pattern-related errors

#[derive(Error, Debug)]
pub enum PatternError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Coordination failed: {0}")]
    CoordinationFailed(String),

    #[error("Formation failed: {0}")]
    FormationFailed(String),

    #[error("Auction error: {0}")]
    AuctionError(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Pattern error: {0}")]
    Other(String),
}
