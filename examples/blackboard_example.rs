use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Blackboard Example ===\n");

    // Create blackboard
    let mut blackboard = Blackboard::new("shared_knowledge");

    // Add knowledge sources
    let sensor = KnowledgeSource::new(AgentId::new(), KnowledgeSourceType::Sensor).with_priority(1);

    let reasoner =
        KnowledgeSource::new(AgentId::new(), KnowledgeSourceType::Reasoning).with_priority(2);

    blackboard.add_source(sensor);
    blackboard.add_source(reasoner);

    // Write knowledge
    blackboard.write("temperature", "25°C");
    blackboard.write("pressure", "1013 hPa");
    blackboard.write("inference", "weather_stable");

    // Read knowledge
    if let Some(temp) = blackboard.read("temperature") {
        println!("Temperature: {}", temp);
    }

    println!("Knowledge items: {}", blackboard.size());
    println!("Sources: {}", blackboard.sources().len());
}
