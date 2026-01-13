//! Federation governance patterns

/// Federation structure
pub mod federation;
/// Federation policies
pub mod policy;

pub use federation::Federation;
pub use policy::{Policy, PolicyType};
