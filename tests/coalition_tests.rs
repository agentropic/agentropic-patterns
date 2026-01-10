use agentropic_patterns::prelude::*;

#[test]
fn create_coalition() {
    let mut coalition = Coalition::new("trading_coalition");

    coalition.add_member(AgentId::new());
    coalition.add_member(AgentId::new());
    coalition.set_value(1000.0);

    assert_eq!(coalition.size(), 2);
    assert_eq!(coalition.value(), 1000.0);
}

#[test]
fn coalition_formation() {
    let mut formation = Formation::new(FormationType::Negotiation);

    let agent1 = AgentId::new();
    let agent2 = AgentId::new();

    formation.add_candidate(agent1.clone());
    formation.add_candidate(agent2.clone());
    formation.select(agent1);

    assert!(formation.is_complete());
    assert_eq!(formation.selected().len(), 1);
}
