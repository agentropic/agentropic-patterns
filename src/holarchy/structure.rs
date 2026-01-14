use super::Holon;
use agentropic_core::AgentId;
use std::collections::HashMap;

/// Holarchic organization structure
#[derive(Debug, Clone)]
pub struct Holarchy {
    name: String,
    holons: HashMap<AgentId, Holon>,
    root: Option<AgentId>,
}

impl Holarchy {
    /// Create a new holarchy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            holons: HashMap::new(),
            root: None,
        }
    }

    /// Add a holon
    pub fn add_holon(&mut self, holon: Holon) {
        let id = *holon.id(); // ← FIXED: Use dereference instead of clone
        self.holons.insert(id, holon); // ← FIXED: Removed .clone()

        // Set as root if first holon
        if self.root.is_none() {
            self.root = Some(id);
        }
    }

    /// Set root holon
    pub fn set_root(&mut self, root: AgentId) {
        if self.holons.contains_key(&root) {
            self.root = Some(root);
        }
    }

    /// Get holon
    pub fn get_holon(&self, id: &AgentId) -> Option<&Holon> {
        self.holons.get(id)
    }

    /// Get mutable holon
    pub fn get_holon_mut(&mut self, id: &AgentId) -> Option<&mut Holon> {
        self.holons.get_mut(id)
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get root holon
    pub fn root(&self) -> Option<&AgentId> {
        self.root.as_ref()
    }

    /// Get all holons
    pub fn holons(&self) -> impl Iterator<Item = &Holon> {
        self.holons.values()
    }

    /// Get number of holons
    pub fn size(&self) -> usize {
        self.holons.len()
    }
}
