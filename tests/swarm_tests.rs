use agentropic_patterns::prelude::*;

#[test]
fn create_swarm() {
    let mut swarm = Swarm::new("robot_swarm");

    swarm.add_member(AgentId::new());
    swarm.add_member(AgentId::new());
    swarm.add_member(AgentId::new());

    assert_eq!(swarm.size(), 3);
}

#[test]
fn flocking_behavior() {
    let flocking = Flocking::new()
        .with_separation(2.0)
        .with_alignment(1.5)
        .with_cohesion(1.0);

    assert_eq!(flocking.separation_weight(), 2.0);
    assert_eq!(flocking.alignment_weight(), 1.5);
    assert_eq!(flocking.cohesion_weight(), 1.0);
}

#[test]
fn consensus() {
    let mut consensus = Consensus::new(0.6);

    consensus.vote(AgentId::new(), "option_a");
    consensus.vote(AgentId::new(), "option_a");
    consensus.vote(AgentId::new(), "option_b");

    assert!(consensus.is_reached());
    assert_eq!(consensus.winner(), Some("option_a".to_string()));
}
