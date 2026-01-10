use agentropic_patterns::prelude::*;

#[test]
fn create_hierarchy() {
    let mut hierarchy = Hierarchy::new("corporate");

    let strategic = Level::new("C-Level", LevelType::Strategic, 3);
    let tactical = Level::new("Management", LevelType::Tactical, 2);
    let operational = Level::new("Workers", LevelType::Operational, 1);

    hierarchy.add_level(strategic.clone());
    hierarchy.add_level(tactical.clone());
    hierarchy.add_level(operational);

    assert_eq!(hierarchy.name(), "corporate");
    assert_eq!(hierarchy.levels().len(), 3);
}

#[test]
fn level_comparison() {
    let high = Level::new("High", LevelType::Strategic, 10);
    let low = Level::new("Low", LevelType::Operational, 1);

    assert!(high.is_above(&low));
    assert!(!low.is_above(&high));
}

#[test]
fn delegation() {
    let manager = AgentId::new();
    let worker = AgentId::new();

    let delegation = Delegation::new(manager.clone(), worker.clone(), "process_orders", 2);

    assert_eq!(delegation.from(), &manager);
    assert_eq!(delegation.to(), &worker);
    assert_eq!(delegation.task(), "process_orders");
}
