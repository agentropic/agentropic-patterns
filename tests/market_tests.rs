use agentropic_patterns::prelude::*;

#[test]
fn create_auction() {
    let mut auction =
        Auction::new(AuctionType::English, "compute_resource").with_reserve_price(100.0);

    let bidder1 = AgentId::new();
    let bidder2 = AgentId::new();

    auction.add_bid(Bid::new(bidder1, 150.0, "compute_resource"));
    auction.add_bid(Bid::new(bidder2, 200.0, "compute_resource"));

    let winner = auction.winner().unwrap();
    assert_eq!(winner.amount(), 200.0);
}

#[test]
fn allocation() {
    let mut allocation = Allocation::new();
    let agent = AgentId::new();

    allocation.allocate(agent.clone(), "cpu_1");
    allocation.allocate(agent.clone(), "memory_2gb");

    let resources = allocation.get(&agent).unwrap();
    assert_eq!(resources.len(), 2);
}
