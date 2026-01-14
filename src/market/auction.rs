use super::Bid;
use serde::{Deserialize, Serialize};

/// Type of auction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuctionType {
    /// English auction (ascending price)
    English,
    /// Dutch auction (descending price)
    Dutch,
    /// Sealed bid auction
    SealedBid,
    /// Vickrey auction (second-price sealed bid)
    Vickrey,
}

/// Auction mechanism
#[derive(Debug, Clone)]
pub struct Auction {
    auction_type: AuctionType,
    resource: String,
    bids: Vec<Bid>,
    reserve_price: Option<f64>,
}

impl Auction {
    /// Create a new auction
    pub fn new(auction_type: AuctionType, resource: impl Into<String>) -> Self {
        Self {
            auction_type,
            resource: resource.into(),
            bids: Vec::new(),
            reserve_price: None,
        }
    }

    /// Set reserve price
    pub fn with_reserve_price(mut self, price: f64) -> Self {
        self.reserve_price = Some(price);
        self
    }

    /// Add a bid
    pub fn add_bid(&mut self, bid: Bid) {
        self.bids.push(bid);
    }

    /// Get winning bid
    pub fn winner(&self) -> Option<&Bid> {
        if self.bids.is_empty() {
            return None;
        }

        let mut sorted = self.bids.clone();
        sorted.sort_by(|a, b| b.amount().partial_cmp(&a.amount()).unwrap());

        let winner = &sorted[0];

        // Check reserve price
        if let Some(reserve) = self.reserve_price {
            if winner.amount() < reserve {
                return None;
            }
        }

        self.bids.iter().find(|b| b.bidder() == winner.bidder())
    }

    /// Get auction type
    pub fn auction_type(&self) -> AuctionType {
        self.auction_type
    }

    /// Get resource
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Get all bids
    pub fn bids(&self) -> &[Bid] {
        &self.bids
    }
}
