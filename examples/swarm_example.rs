use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Swarm Example ===\n");

    // Create swarm
    let mut swarm = Swarm::new("drone_swarm");

    // Add members
    for _ in 0..10 {
        swarm.add_member(AgentId::new());
    }

    // Set flocking behavior
    let behavior = Behavior::new(BehaviorType::Flocking)
        .with_parameter("separation", 2.0)
        .with_parameter("alignment", 1.0)
        .with_parameter("cohesion", 1.0);

    swarm.set_behavior(behavior);

    println!("Swarm: {}", swarm.name());
    println!("Size: {}", swarm.size());
    println!("Behavior: {:?}", swarm.behavior().unwrap().behavior_type());

    // Consensus decision
    let mut consensus = Consensus::new(0.7);
    for member in swarm.members() {
        consensus.vote(member.clone(), "target_north");
    }

    if consensus.is_reached() {
        println!("Consensus reached: {:?}", consensus.winner());
    }
}
