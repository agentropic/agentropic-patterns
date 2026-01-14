/// Foraging behavior (ant colony optimization)
#[derive(Debug, Clone)]
pub struct Foraging {
    pheromone_strength: f64,
    evaporation_rate: f64,
    exploration_rate: f64,
}

impl Foraging {
    /// Create new foraging behavior
    pub fn new() -> Self {
        Self {
            pheromone_strength: 1.0,
            evaporation_rate: 0.1,
            exploration_rate: 0.2,
        }
    }

    /// Set pheromone strength
    pub fn with_pheromone_strength(mut self, strength: f64) -> Self {
        self.pheromone_strength = strength;
        self
    }

    /// Set evaporation rate
    pub fn with_evaporation_rate(mut self, rate: f64) -> Self {
        self.evaporation_rate = rate;
        self
    }

    /// Set exploration rate
    pub fn with_exploration_rate(mut self, rate: f64) -> Self {
        self.exploration_rate = rate;
        self
    }

    /// Get pheromone strength
    pub fn pheromone_strength(&self) -> f64 {
        self.pheromone_strength
    }

    /// Get evaporation rate
    pub fn evaporation_rate(&self) -> f64 {
        self.evaporation_rate
    }

    /// Get exploration rate
    pub fn exploration_rate(&self) -> f64 {
        self.exploration_rate
    }
}

impl Default for Foraging {
    fn default() -> Self {
        Self::new()
    }
}
