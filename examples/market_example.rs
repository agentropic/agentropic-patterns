use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Market Example ===\n");

    // Create market
    let mut market = Market::new("compute_market");

    // Create auction
    let mut auction = Auction::new(AuctionType::English, "gpu_hours").with_reserve_price(50.0);

    // Add bids
    auction.add_bid(Bid::new(AgentId::new(), 75.0, "gpu_hours"));
    auction.add_bid(Bid::new(AgentId::new(), 100.0, "gpu_hours"));
    auction.add_bid(Bid::new(AgentId::new(), 90.0, "gpu_hours"));

    // Determine winner
    if let Some(winner) = auction.winner() {
        println!("Winner bid: ${}", winner.amount());
        // Allocate resource
        market
            .allocation_mut()
            .allocate(winner.bidder().clone(), "gpu_hours");
    }

    market.add_auction(auction);
    println!("Market: {}", market.name());
    println!("Auctions: {}", market.auctions().len());
}
