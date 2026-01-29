//! Dialogue System
//! 
//! NPC dialogue and conversation management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dialogue node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub portrait: Option<String>,
    pub responses: Vec<DialogueResponse>,
    pub on_enter: Option<String>,  // Script to run when entering this node
    pub on_exit: Option<String>,   // Script to run when leaving this node
}

/// Player response option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub next_node: Option<String>,  // None = end dialogue
    pub condition: Option<String>,   // Condition script
    pub effect: Option<String>,      // Effect script
}

impl DialogueResponse {
    pub fn simple(text: &str, next_node: Option<&str>) -> Self {
        Self {
            text: text.to_string(),
            next_node: next_node.map(|s| s.to_string()),
            condition: None,
            effect: None,
        }
    }
    
    pub fn with_condition(mut self, condition: &str) -> Self {
        self.condition = Some(condition.to_string());
        self
    }
    
    pub fn with_effect(mut self, effect: &str) -> Self {
        self.effect = Some(effect.to_string());
        self
    }
}

/// Complete dialogue tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: String,
    pub nodes: HashMap<String, DialogueNode>,
    pub start_node: String,
}

impl Dialogue {
    pub fn new(id: &str, start_node: &str) -> Self {
        Self {
            id: id.to_string(),
            nodes: HashMap::new(),
            start_node: start_node.to_string(),
        }
    }
    
    pub fn add_node(&mut self, node: DialogueNode) {
        self.nodes.insert(node.id.clone(), node);
    }
    
    pub fn get_node(&self, id: &str) -> Option<&DialogueNode> {
        self.nodes.get(id)
    }
    
    pub fn get_start_node(&self) -> Option<&DialogueNode> {
        self.nodes.get(&self.start_node)
    }
}

/// Dialogue manager handles the current conversation state
#[derive(Debug)]
pub struct DialogueManager {
    current_dialogue: Option<Dialogue>,
    current_node_id: Option<String>,
    dialogue_history: Vec<String>,
    variables: HashMap<String, String>,
}

impl DialogueManager {
    pub fn new() -> Self {
        Self {
            current_dialogue: None,
            current_node_id: None,
            dialogue_history: Vec::new(),
            variables: HashMap::new(),
        }
    }
    
    /// Start a new dialogue
    pub fn start_dialogue(&mut self, dialogue: Dialogue) {
        let start = dialogue.start_node.clone();
        self.current_dialogue = Some(dialogue);
        self.current_node_id = Some(start);
        self.dialogue_history.clear();
    }
    
    /// End the current dialogue
    pub fn end_dialogue(&mut self) {
        self.current_dialogue = None;
        self.current_node_id = None;
    }
    
    /// Is there an active dialogue?
    pub fn is_active(&self) -> bool {
        self.current_dialogue.is_some() && self.current_node_id.is_some()
    }
    
    /// Get the current dialogue node
    pub fn current_node(&self) -> Option<&DialogueNode> {
        let dialogue = self.current_dialogue.as_ref()?;
        let node_id = self.current_node_id.as_ref()?;
        dialogue.get_node(node_id)
    }
    
    /// Select a response and advance the dialogue
    pub fn select_response(&mut self, response_index: usize) -> DialogueResult {
        let node = match self.current_node() {
            Some(n) => n.clone(),
            None => return DialogueResult::NotInDialogue,
        };
        
        if response_index >= node.responses.len() {
            return DialogueResult::InvalidResponse;
        }
        
        let response = &node.responses[response_index];
        
        // Record in history
        self.dialogue_history.push(node.id.clone());
        
        // Move to next node or end
        match &response.next_node {
            Some(next) => {
                self.current_node_id = Some(next.clone());
                DialogueResult::Continue
            }
            None => {
                self.end_dialogue();
                DialogueResult::End
            }
        }
    }
    
    /// Set a dialogue variable
    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
    
    /// Get a dialogue variable
    pub fn get_variable(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }
    
    /// Replace variables in text
    pub fn process_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (key, value) in &self.variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
}

impl Default for DialogueManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of advancing dialogue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueResult {
    Continue,
    End,
    InvalidResponse,
    NotInDialogue,
}

/// NPC component for entities that can have dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub name: String,
    pub dialogue_id: String,
    pub interaction_radius: f32,
    pub facing_player: bool,
}

impl NPC {
    pub fn new(name: &str, dialogue_id: &str) -> Self {
        Self {
            name: name.to_string(),
            dialogue_id: dialogue_id.to_string(),
            interaction_radius: 50.0,
            facing_player: true,
        }
    }
}
