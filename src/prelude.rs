//! Prelude for convenient imports

pub use crate::blackboard::{Blackboard, KnowledgeSource, KnowledgeSourceType};
pub use crate::coalition::{Coalition, Formation, FormationType, Strategy, StrategyType};
pub use crate::federation::{Federation, Policy, PolicyType};
pub use crate::hierarchy::{Delegation, Hierarchy, Level, LevelType};
pub use crate::holarchy::{Holarchy, Holon, HolonType};
pub use crate::market::{Allocation, Auction, AuctionType, Bid, Market};
pub use crate::swarm::{Behavior, BehaviorType, Consensus, Flocking, Foraging, Swarm};
pub use crate::team::{Coordination, Role, RoleType, Team};
pub use crate::PatternError;

// Re-export from core
pub use agentropic_core::prelude::*;
