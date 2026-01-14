//! Holarchic organization patterns

/// Holon (autonomous unit)
pub mod holon;
/// Holarchy structure
pub mod structure;

pub use holon::{Holon, HolonType};
pub use structure::Holarchy;
