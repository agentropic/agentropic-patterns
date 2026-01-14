use super::{Allocation, Auction};

/// Market coordination mechanism
#[derive(Debug, Clone)]
pub struct Market {
    name: String,
    auctions: Vec<Auction>,
    allocation: Allocation,
}

impl Market {
    /// Create a new market
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            auctions: Vec::new(),
            allocation: Allocation::new(),
        }
    }

    /// Add an auction
    pub fn add_auction(&mut self, auction: Auction) {
        self.auctions.push(auction);
    }

    /// Get market name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get auctions
    pub fn auctions(&self) -> &[Auction] {
        &self.auctions
    }

    /// Get allocation
    pub fn allocation(&self) -> &Allocation {
        &self.allocation
    }

    /// Get mutable allocation
    pub fn allocation_mut(&mut self) -> &mut Allocation {
        &mut self.allocation
    }
}
