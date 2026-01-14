//! Market-based coordination patterns

/// Resource allocation
pub mod allocation;
/// Auction mechanisms
pub mod auction;
/// Bid structure
pub mod bid;
/// Market structure
pub mod market;

pub use allocation::Allocation;
pub use auction::{Auction, AuctionType};
pub use bid::Bid;
pub use market::Market;
