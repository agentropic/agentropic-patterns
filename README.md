# agentropic-patterns

[![Crates.io](https://img.shields.io/crates/v/agentropic-patterns.svg)](https://crates.io/crates/agentropic-patterns)
[![Documentation](https://docs.rs/agentropic-patterns/badge.svg)](https://docs.rs/agentropic-patterns)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Multi-agent system coordination patterns and organizational structures.**

`agentropic-patterns` provides high-level organizational patterns and coordination mechanisms for building sophisticated multi-agent systems. It implements proven architectural patterns for agent collaboration, from hierarchical command structures to emergent swarm behaviors.

---

## Purpose

This crate provides:

- **Organizational Patterns**: Hierarchy, holarchy, teams, coalitions, federation
- **Coordination Mechanisms**: Swarm intelligence, market-based coordination
- **Emergent Behaviors**: Flocking, foraging, consensus
- **Resource Allocation**: Auctions, voting, blackboard systems

---

## Implementation Status

**Fully Implemented (v0.1.0)**:
- **Hierarchy** - Multi-level organizational structures with delegation
- **Team** - Role-based peer collaboration
- **Swarm** - Flocking, foraging, and consensus mechanisms
- **Market** - Auction-based resource allocation with multiple auction types
- **Coalition** - Dynamic group formation with strategies
- **Holarchy** - Nested autonomous units
- **Federation** - Policy-based distributed coordination
- **Blackboard** - Shared knowledge space with knowledge sources

All patterns have:
-  Working implementations
-  Comprehensive tests
-  Runnable examples
-  Full documentation

---

## Core Patterns

### 1. Hierarchical Organization

Command and control structure with clear authority lines:
```rust
use agentropic_patterns::prelude::*;

// Create hierarchy
let mut hierarchy = Hierarchy::new("corporate");

// Add organizational levels
let ceo_level = Level::new("CEO", LevelType::Strategic, 3);
let manager_level = Level::new("Manager", LevelType::Tactical, 2);
let worker_level = Level::new("Worker", LevelType::Operational, 1);

hierarchy.add_level(ceo_level.clone());
hierarchy.add_level(manager_level.clone());
hierarchy.add_level(worker_level);

// Assign agents to levels
let ceo = AgentId::new();
let manager = AgentId::new();
let worker = AgentId::new();

hierarchy.assign_agent(ceo, ceo_level);
hierarchy.assign_agent(manager, manager_level);
hierarchy.assign_agent(worker, worker_level);

// Delegate tasks
let delegation = Delegation::new(ceo, manager, "implement_strategy", 2);
hierarchy.delegate(delegation);
```

### 2. Team Organization

Peer-based collaboration with shared goals:
```rust
use agentropic_patterns::prelude::*;

// Create a team
let mut team = Team::new("development-team");

// Add members with roles
let leader_role = Role::new("team_leader", RoleType::Leader);
let member_role = Role::new("developer", RoleType::Executor);

team.assign_role(agent1, leader_role);
team.assign_role(agent2, member_role);
team.assign_role(agent3, member_role);

// Set team leader
team.set_leader(agent1);

println!("Team: {}", team.name());
println!("Members: {}", team.members().len());
```

### 3. Swarm Intelligence

Decentralized, emergent behavior through local interactions:
```rust
use agentropic_patterns::prelude::*;

// Create a swarm
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

// Consensus decision-making
let mut consensus = Consensus::new(0.7); // 70% threshold

for member in swarm.members() {
    consensus.vote(*member, "target_north");
}

if consensus.is_reached() {
    println!("Swarm consensus: {:?}", consensus.winner());
}
```

### 4. Market-Based Coordination

Resource allocation through bidding and pricing:
```rust
use agentropic_patterns::prelude::*;

// Create a market
let mut market = Market::new("compute_market");

// Create auction
let mut auction = Auction::new(AuctionType::English, "gpu_hours")
    .with_reserve_price(100.0);

// Agents submit bids
let bidder1 = AgentId::new();
let bidder2 = AgentId::new();
let bidder3 = AgentId::new();

auction.add_bid(Bid::new(bidder1, 150.0, "gpu_hours"));
auction.add_bid(Bid::new(bidder2, 200.0, "gpu_hours"));
auction.add_bid(Bid::new(bidder3, 175.0, "gpu_hours"));

// Determine winner
if let Some(winner) = auction.winner() {
    println!("Winner: Agent with bid ${}", winner.amount());
    
    // Allocate resource
    market.allocation_mut().allocate(*winner.bidder(), "gpu_hours");
}

market.add_auction(auction);
```

### 5. Coalition Formation

Dynamic grouping based on shared interests:
```rust
use agentropic_patterns::prelude::*;

// Create coalition
let mut coalition = Coalition::new("trading_coalition");

// Add members
coalition.add_member(AgentId::new());
coalition.add_member(AgentId::new());
coalition.add_member(AgentId::new());

// Set strategy
let strategy = Strategy::new(StrategyType::MaximizeUtility)
    .with_parameter("risk_tolerance", 0.6);

coalition.set_strategy(strategy);
coalition.set_value(5000.0);

// Coalition formation process
let mut formation = Formation::new(FormationType::Negotiation);

let agent1 = AgentId::new();
let agent2 = AgentId::new();

formation.add_candidate(agent1);
formation.add_candidate(agent2);
formation.select(agent1);

println!("Coalition size: {}", coalition.size());
println!("Formation complete: {}", formation.is_complete());
```

### 6. Blackboard System

Shared knowledge space for problem-solving:
```rust
use agentropic_patterns::prelude::*;

// Create blackboard
let mut blackboard = Blackboard::new("shared_knowledge");

// Add knowledge sources
let sensor = KnowledgeSource::new(
    AgentId::new(),
    KnowledgeSourceType::Sensor,
).with_priority(1);

let reasoner = KnowledgeSource::new(
    AgentId::new(),
    KnowledgeSourceType::Reasoning,
).with_priority(2);

blackboard.add_source(sensor);
blackboard.add_source(reasoner);

// Write and read knowledge
blackboard.write("temperature", "25°C");
blackboard.write("pressure", "1013 hPa");

if let Some(temp) = blackboard.read("temperature") {
    println!("Temperature: {}", temp);
}

println!("Knowledge items: {}", blackboard.size());
```

### 7. Holarchy

Nested hierarchy where each unit is both whole and part:
```rust
use agentropic_patterns::prelude::*;

// Create holarchy
let mut holarchy = Holarchy::new("organization");

// Create holons (autonomous units)
let company = Holon::composite(AgentId::new());
let engineering = Holon::composite(AgentId::new());
let frontend = Holon::atomic(AgentId::new());

// Build nested structure
holarchy.add_holon(company);
holarchy.add_holon(engineering);
holarchy.add_holon(frontend);

// Set root
holarchy.set_root(*company.id());

println!("Holarchy: {}", holarchy.name());
println!("Holons: {}", holarchy.size());
```

### 8. Federation

Autonomous units with coordinated policies:
```rust
use agentropic_patterns::prelude::*;

// Create federation
let mut federation = Federation::new("global_federation");

// Add autonomous members
let org1 = AgentId::new();
let org2 = AgentId::new();

federation.add_member(org1);
federation.add_member(org2);

// Set member weights for voting
federation.set_weight(org1, 1.5);
federation.set_weight(org2, 1.0);

// Add coordination policies
let policy = Policy::new("data_sharing", PolicyType::Consensus)
    .with_threshold(0.66)
    .with_rule("must_anonymize");

federation.add_policy(policy);

println!("Federation: {}", federation.name());
println!("Members: {}", federation.size());
```

---

## What's Included

### Organizational Patterns

- `Hierarchy` - Multi-level command structures with delegation
- `Level` - Organizational levels (Strategic, Tactical, Operational)
- `Delegation` - Task delegation between levels
- `Team` - Peer-based collaboration with roles
- `Role` - Team member roles (Leader, Coordinator, Executor, Specialist)
- `Coordination` - Team coordination mechanisms

### Swarm Intelligence

- `Swarm` - Collective behavior coordination
- `Behavior` - Swarm behaviors (Flocking, Foraging, Exploration, Aggregation)
- `Flocking` - Reynolds' boids algorithm (separation, alignment, cohesion)
- `Foraging` - Ant colony optimization patterns
- `Consensus` - Swarm consensus mechanisms with configurable thresholds

### Market-Based Coordination

- `Market` - Economic coordination mechanism
- `Auction` - Auction types (English, Dutch, SealedBid, Vickrey)
- `Bid` - Bidding structure
- `Allocation` - Resource allocation tracking

### Coalition & Collaboration

- `Coalition` - Dynamic group formation
- `Formation` - Formation algorithms (TopDown, BottomUp, Negotiation, Auction)
- `Strategy` - Coalition strategies (MaximizeUtility, MinimizeCost, BalanceResources, MaximizeCoverage)
- `Holarchy` - Nested autonomous units
- `Holon` - Autonomous unit (Atomic or Composite)

### Federation & Governance

- `Federation` - Coordinated autonomous organizations
- `Policy` - Federation policies (Consensus, MajorityVote, WeightedVote, Democratic)
- `Blackboard` - Shared knowledge space
- `KnowledgeSource` - Knowledge contributors (Sensor, Reasoning, Planning, Learning)

---

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
agentropic-patterns = "0.1.0"
agentropic-core = "0.1.0"
agentropic-messaging = "0.1.0"
```

### Complete Swarm Example
```rust
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
        consensus.vote(*member, "target_north");
    }
    
    if consensus.is_reached() {
        println!("Consensus reached: {:?}", consensus.winner());
    }
}
```

### Market-Based Task Allocation
```rust
use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Market Example ===\n");
    
    // Create market
    let mut market = Market::new("compute_market");
    
    // Create auction
    let mut auction = Auction::new(AuctionType::English, "gpu_hours")
        .with_reserve_price(50.0);
    
    // Add bids
    auction.add_bid(Bid::new(AgentId::new(), 75.0, "gpu_hours"));
    auction.add_bid(Bid::new(AgentId::new(), 100.0, "gpu_hours"));
    auction.add_bid(Bid::new(AgentId::new(), 90.0, "gpu_hours"));
    
    // Determine winner
    if let Some(winner) = auction.winner() {
        println!("Winner bid: ${}", winner.amount());
        
        // Allocate resource
        market.allocation_mut().allocate(
            *winner.bidder(),
            "gpu_hours",
        );
    }
    
    market.add_auction(auction);
    println!("Market: {}", market.name());
    println!("Auctions: {}", market.auctions().len());
}
```

### Hierarchical Organization
```rust
use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Hierarchy Example ===\n");
    
    // Create hierarchy
    let mut hierarchy = Hierarchy::new("corporate");
    
    // Define levels
    let ceo_level = Level::new("CEO", LevelType::Strategic, 3);
    let manager_level = Level::new("Manager", LevelType::Tactical, 2);
    let worker_level = Level::new("Worker", LevelType::Operational, 1);
    
    hierarchy.add_level(ceo_level.clone());
    hierarchy.add_level(manager_level.clone());
    hierarchy.add_level(worker_level.clone());
    
    // Assign agents
    let ceo = AgentId::new();
    let manager = AgentId::new();
    let worker = AgentId::new();
    
    hierarchy.assign_agent(ceo, ceo_level);
    hierarchy.assign_agent(manager, manager_level);
    hierarchy.assign_agent(worker, worker_level);
    
    // Delegate task
    let delegation = Delegation::new(ceo, manager, "implement_strategy", 2);
    hierarchy.delegate(delegation);
    
    println!("Hierarchy: {}", hierarchy.name());
    println!("Levels: {}", hierarchy.levels().len());
    println!("Delegations: {}", hierarchy.delegations().len());
}
```

### Coalition Formation
```rust
use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Coalition Example ===\n");
    
    // Create coalition
    let mut coalition = Coalition::new("defi_coalition");
    
    // Add members
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    let agent3 = AgentId::new();
    
    coalition.add_member(agent1);
    coalition.add_member(agent2);
    coalition.add_member(agent3);
    
    // Set strategy
    let strategy = Strategy::new(StrategyType::MaximizeUtility)
        .with_parameter("risk_tolerance", 0.6);
    
    coalition.set_strategy(strategy);
    coalition.set_value(5000.0);
    
    println!("Coalition: {}", coalition.name());
    println!("Size: {}", coalition.size());
    println!("Value: ${}", coalition.value());
}
```

---

## Architecture

### Pattern Categories

**Structural Patterns** - How agents are organized:
- Hierarchy, holarchy, team, federation

**Behavioral Patterns** - How agents interact:
- Swarm, market, blackboard

**Coordination Patterns** - How agents synchronize:
- Consensus, voting, auction, coalition

### Pattern Selection Guide

| Use Case | Pattern | Characteristics |
|----------|---------|----------------|
| Clear command structure | Hierarchy | Centralized, efficient, clear authority |
| Peer collaboration | Team | Distributed, flexible, role-based |
| Large-scale coordination | Swarm | Emergent, scalable, decentralized |
| Resource allocation | Market | Economic, competitive, fair |
| Shared problem-solving | Blackboard | Collaborative, opportunistic |
| Temporary collaboration | Coalition | Dynamic, goal-oriented, strategic |
| Autonomous units | Federation | Independent, coordinated policies |
| Nested autonomy | Holarchy | Hierarchical but autonomous at each level |

### Implementation Architecture
```
┌─────────────────────────────────────────┐
│       Pattern Layer (High-Level)        │
├─────────────────────────────────────────┤
│  Hierarchy │ Team │ Swarm │ Market      │
│  Coalition │ Federation │ Blackboard    │
├─────────────────────────────────────────┤
│         Messaging Layer                 │
│  (agentropic-messaging)                 │
├─────────────────────────────────────────┤
│         Core Layer                      │
│  (agentropic-core)                      │
└─────────────────────────────────────────┘
```

---

## Examples

See the [examples](examples/) directory for complete, runnable examples:

- `hierarchy_example.rs` - Corporate organizational structure
- `swarm_example.rs` - Drone swarm with flocking and consensus
- `market_example.rs` - Auction-based resource allocation
- `coalition_example.rs` - Dynamic coalition formation
- `blackboard_example.rs` - Shared knowledge system

Run examples:
```bash
cargo run --example hierarchy_example
cargo run --example swarm_example
cargo run --example market_example
cargo run --example coalition_example
cargo run --example blackboard_example
```

---

## 🔗 Related Crates

- **[agentropic-core](../agentropic-core)** - Agent primitives and lifecycle
- **[agentropic-messaging](../agentropic-messaging)** - Communication protocols
- **[agentropic-cognition](../agentropic-cognition)** - BDI reasoning and planning
- **[agentropic-runtime](../agentropic-runtime)** - Agent execution engine

---

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/agentropic-patterns).

For guides and tutorials, see the [Agentropic documentation](https://github.com/agentropic/agentropic-docs).

---

## References

This crate is inspired by academic research in multi-agent systems:

- **Swarm Intelligence** - Kennedy & Eberhart (1995) - Particle swarm optimization
- **Market-Based Control** - Clearwater (1996) - Economic coordination mechanisms
- **Contract Net Protocol** - Smith (1980) - Task allocation through negotiation
- **Blackboard Systems** - Erman et al. (1980) - Opportunistic problem-solving
- **Holonic Systems** - Koestler (1967) - Nested autonomous systems
- **Coalition Formation** - Shehory & Kraus (1998) - Dynamic group formation algorithms
- **FIPA Standards** - Foundation for Intelligent Physical Agents - Agent communication

---

## Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTING.md).

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Status

**Active Development** - v0.1.0 released with all 8 core patterns fully implemented and tested.

**Roadmap**:
- v0.2.0: Builder pattern API for ergonomic construction
- v0.3.0: Advanced patterns (matrix organization, stigmergy)
- v0.4.0: Distributed patterns across network nodes
- v1.0.0: Stable API with comprehensive benchmarks

---

*Part of the [Agentropic](https://github.com/agentropic) ecosystem for agent-oriented programming in Rust.*
