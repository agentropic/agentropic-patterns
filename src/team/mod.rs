//! Team-based collaboration patterns

/// Team coordination
pub mod coordination;
/// Role definitions
pub mod role;
/// Team structure
pub mod team;

pub use coordination::Coordination;
pub use role::{Role, RoleType};
pub use team::Team;
