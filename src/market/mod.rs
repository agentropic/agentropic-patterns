//! Market-based coordination patterns

/// Resource allocation
pub mod allocation;
/// Auction mechanisms
pub mod auction;
/// Bid structure
pub mod bid;
/// Market structure
pub mod structure;

pub use allocation::Allocation;
pub use auction::{Auction, AuctionType};
pub use bid::Bid;
pub use structure::Market;
