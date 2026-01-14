/// Flocking behavior (Reynolds' Boids)
#[derive(Debug, Clone)]
pub struct Flocking {
    separation_weight: f64,
    alignment_weight: f64,
    cohesion_weight: f64,
}

impl Flocking {
    /// Create new flocking behavior
    pub fn new() -> Self {
        Self {
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        }
    }

    /// Set separation weight
    pub fn with_separation(mut self, weight: f64) -> Self {
        self.separation_weight = weight;
        self
    }

    /// Set alignment weight
    pub fn with_alignment(mut self, weight: f64) -> Self {
        self.alignment_weight = weight;
        self
    }

    /// Set cohesion weight
    pub fn with_cohesion(mut self, weight: f64) -> Self {
        self.cohesion_weight = weight;
        self
    }

    /// Get separation weight
    pub fn separation_weight(&self) -> f64 {
        self.separation_weight
    }

    /// Get alignment weight
    pub fn alignment_weight(&self) -> f64 {
        self.alignment_weight
    }

    /// Get cohesion weight
    pub fn cohesion_weight(&self) -> f64 {
        self.cohesion_weight
    }
}

impl Default for Flocking {
    fn default() -> Self {
        Self::new()
    }
}
