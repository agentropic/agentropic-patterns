use agentropic_core::AgentId;
use serde::{Deserialize, Serialize};

/// Type of holon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HolonType {
    /// Atomic holon (single agent)
    Atomic,
    /// Composite holon (contains sub-holons)
    Composite,
}

/// Holon - autonomous unit that can be part of larger whole
#[derive(Debug, Clone)]
pub struct Holon {
    id: AgentId,
    holon_type: HolonType,
    parent: Option<AgentId>,
    children: Vec<AgentId>,
    autonomy_level: f64,
}

impl Holon {
    /// Create a new atomic holon
    pub fn atomic(id: AgentId) -> Self {
        Self {
            id,
            holon_type: HolonType::Atomic,
            parent: None,
            children: Vec::new(),
            autonomy_level: 1.0,
        }
    }

    /// Create a new composite holon
    pub fn composite(id: AgentId) -> Self {
        Self {
            id,
            holon_type: HolonType::Composite,
            parent: None,
            children: Vec::new(),
            autonomy_level: 0.8,
        }
    }

    /// Set parent holon
    pub fn set_parent(&mut self, parent: AgentId) {
        self.parent = Some(parent);
    }

    /// Add child holon
    pub fn add_child(&mut self, child: AgentId) {
        self.children.push(child);
        if self.holon_type == HolonType::Atomic {
            self.holon_type = HolonType::Composite;
        }
    }

    /// Set autonomy level (0.0 to 1.0)
    pub fn set_autonomy_level(&mut self, level: f64) {
        self.autonomy_level = level.clamp(0.0, 1.0);
    }

    /// Get ID
    pub fn id(&self) -> &AgentId {
        &self.id
    }

    /// Get holon type
    pub fn holon_type(&self) -> HolonType {
        self.holon_type
    }

    /// Get parent
    pub fn parent(&self) -> Option<&AgentId> {
        self.parent.as_ref()
    }

    /// Get children
    pub fn children(&self) -> &[AgentId] {
        &self.children
    }

    /// Get autonomy level
    pub fn autonomy_level(&self) -> f64 {
        self.autonomy_level
    }

    /// Check if this is a leaf holon (no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}
