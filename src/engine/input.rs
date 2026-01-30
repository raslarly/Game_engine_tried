//! Input Management Module
//! 
//! Handles keyboard, mouse, and gamepad input.

use egui::{Key, Modifiers, PointerButton};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Input action binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAction {
    pub name: String,
    pub keys: Vec<KeyBinding>,
    pub mouse_buttons: Vec<MouseBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: InputModifiers,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseBinding {
    pub button: String, // "left", "right", "middle"
}

/// Axis binding for analog input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAxis {
    pub name: String,
    pub positive_keys: Vec<String>,
    pub negative_keys: Vec<String>,
    pub sensitivity: f32,
    pub gravity: f32, // How fast the axis returns to 0
}

/// Input manager handles all input state
pub struct InputManager {
    // Current frame state
    keys_down: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,
    
    // Mouse buttons stored as list since Hash is not implemented for PointerButton
    mouse_buttons_down: Vec<PointerButton>,
    mouse_buttons_pressed: Vec<PointerButton>,
    mouse_buttons_released: Vec<PointerButton>,
    
    mouse_position: Vec2,
    mouse_delta: Vec2,
    scroll_delta: Vec2,
    
    // Modifiers
    modifiers: Modifiers,
    
    // Action bindings
    actions: HashMap<String, InputAction>,
    axes: HashMap<String, InputAxis>,
    axis_values: HashMap<String, f32>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_buttons_down: Vec::new(),
            mouse_buttons_pressed: Vec::new(),
            mouse_buttons_released: Vec::new(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: Vec2::ZERO,
            modifiers: Modifiers::default(),
            actions: HashMap::new(),
            axes: HashMap::new(),
            axis_values: HashMap::new(),
        }
    }
    
    /// Update input state at the start of each frame
    pub fn update(&mut self, ctx: &egui::Context) {
        // Clear per-frame events
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
        
        // Update from egui input
        ctx.input(|i| {
            self.modifiers = i.modifiers;
            
            // Update mouse position
            if let Some(pos) = i.pointer.hover_pos() {
                let new_pos = Vec2::new(pos.x, pos.y);
                self.mouse_delta = new_pos - self.mouse_position;
                self.mouse_position = new_pos;
            }
            
            // Update scroll
            self.scroll_delta = Vec2::new(i.raw_scroll_delta.x, i.raw_scroll_delta.y);
            
            // Process keyboard events
            for event in &i.events {
                match event {
                    egui::Event::Key { key, pressed, .. } => {
                        if *pressed {
                            if self.keys_down.insert(*key) {
                                self.keys_pressed.insert(*key);
                            }
                        } else {
                            if self.keys_down.remove(key) {
                                self.keys_released.insert(*key);
                            }
                        }
                    }
                    egui::Event::PointerButton { button, pressed, .. } => {
                        if *pressed {
                            if !self.mouse_buttons_down.contains(button) {
                                self.mouse_buttons_down.push(*button);
                                self.mouse_buttons_pressed.push(*button);
                            }
                        } else {
                            if let Some(idx) = self.mouse_buttons_down.iter().position(|b| b == button) {
                                self.mouse_buttons_down.remove(idx);
                                self.mouse_buttons_released.push(*button);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
        
        // Update axis values
        self.update_axes();
    }
    
    fn update_axes(&mut self) {
        for (name, axis) in &self.axes {
            let mut value = *self.axis_values.get(name).unwrap_or(&0.0);
            
            let positive = axis.positive_keys.iter()
                .any(|k| self.is_key_string_down(k));
            let negative = axis.negative_keys.iter()
                .any(|k| self.is_key_string_down(k));
            
            if positive && !negative {
                value = (value + axis.sensitivity).min(1.0);
            } else if negative && !positive {
                value = (value - axis.sensitivity).max(-1.0);
            } else {
                // Apply gravity toward 0
                if value > 0.0 {
                    value = (value - axis.gravity).max(0.0);
                } else if value < 0.0 {
                    value = (value + axis.gravity).min(0.0);
                }
            }
            
            self.axis_values.insert(name.clone(), value);
        }
    }
    
    fn is_key_string_down(&self, key_str: &str) -> bool {
        if let Some(key) = string_to_key(key_str) {
            self.keys_down.contains(&key)
        } else {
            false
        }
    }
    
    // Key queries
    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }
    
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }
    
    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }
    
    // Mouse queries
    pub fn is_mouse_button_down(&self, button: PointerButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }
    
    pub fn is_mouse_button_pressed(&self, button: PointerButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }
    
    pub fn is_mouse_button_released(&self, button: PointerButton) -> bool {
        self.mouse_buttons_released.contains(&button)
    }
    
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
    
    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }
    
    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }
    
    // Modifiers
    pub fn is_ctrl_down(&self) -> bool {
        self.modifiers.ctrl
    }
    
    pub fn is_shift_down(&self) -> bool {
        self.modifiers.shift
    }
    
    pub fn is_alt_down(&self) -> bool {
        self.modifiers.alt
    }
    
    // Action system
    pub fn register_action(&mut self, action: InputAction) {
        self.actions.insert(action.name.clone(), action);
    }
    
    pub fn register_axis(&mut self, axis: InputAxis) {
        self.axis_values.insert(axis.name.clone(), 0.0);
        self.axes.insert(axis.name.clone(), axis);
    }
    
    pub fn is_action_pressed(&self, name: &str) -> bool {
        if let Some(action) = self.actions.get(name) {
            for binding in &action.keys {
                if let Some(key) = string_to_key(&binding.key) {
                    if self.keys_pressed.contains(&key) {
                        // Check modifiers
                        let mods_match = 
                            (!binding.modifiers.ctrl || self.modifiers.ctrl) &&
                            (!binding.modifiers.shift || self.modifiers.shift) &&
                            (!binding.modifiers.alt || self.modifiers.alt);
                        
                        if mods_match {
                            return true;
                        }
                    }
                }
            }
            
            for binding in &action.mouse_buttons {
                if let Some(button) = string_to_mouse_button(&binding.button) {
                    if self.mouse_buttons_pressed.contains(&button) {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    pub fn is_action_down(&self, name: &str) -> bool {
        if let Some(action) = self.actions.get(name) {
            for binding in &action.keys {
                if let Some(key) = string_to_key(&binding.key) {
                    if self.keys_down.contains(&key) {
                        let mods_match = 
                            (!binding.modifiers.ctrl || self.modifiers.ctrl) &&
                            (!binding.modifiers.shift || self.modifiers.shift) &&
                            (!binding.modifiers.alt || self.modifiers.alt);
                        
                        if mods_match {
                            return true;
                        }
                    }
                }
            }
            
            for binding in &action.mouse_buttons {
                if let Some(button) = string_to_mouse_button(&binding.button) {
                    if self.mouse_buttons_down.contains(&button) {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    pub fn get_axis(&self, name: &str) -> f32 {
        *self.axis_values.get(name).unwrap_or(&0.0)
    }
    
    /// Create default input bindings for action-adventure games
    pub fn setup_default_bindings(&mut self) {
        // Movement axes
        self.register_axis(InputAxis {
            name: "horizontal".to_string(),
            positive_keys: vec!["D".to_string(), "ArrowRight".to_string()],
            negative_keys: vec!["A".to_string(), "ArrowLeft".to_string()],
            sensitivity: 0.1,
            gravity: 0.2,
        });
        
        self.register_axis(InputAxis {
            name: "vertical".to_string(),
            positive_keys: vec!["W".to_string(), "ArrowUp".to_string()],
            negative_keys: vec!["S".to_string(), "ArrowDown".to_string()],
            sensitivity: 0.1,
            gravity: 0.2,
        });
        
        // Common actions
        self.register_action(InputAction {
            name: "jump".to_string(),
            keys: vec![
                KeyBinding { key: "Space".to_string(), modifiers: InputModifiers::default() },
            ],
            mouse_buttons: vec![],
        });
        
        self.register_action(InputAction {
            name: "attack".to_string(),
            keys: vec![
                KeyBinding { key: "J".to_string(), modifiers: InputModifiers::default() },
            ],
            mouse_buttons: vec![
                MouseBinding { button: "left".to_string() },
            ],
        });
        
        self.register_action(InputAction {
            name: "interact".to_string(),
            keys: vec![
                KeyBinding { key: "E".to_string(), modifiers: InputModifiers::default() },
            ],
            mouse_buttons: vec![],
        });
        
        self.register_action(InputAction {
            name: "dash".to_string(),
            keys: vec![
                KeyBinding { key: "K".to_string(), modifiers: InputModifiers::default() },
                KeyBinding { 
                    key: "Space".to_string(), 
                    modifiers: InputModifiers { shift: true, ..Default::default() },
                },
            ],
            mouse_buttons: vec![],
        });
        
        self.register_action(InputAction {
            name: "pause".to_string(),
            keys: vec![
                KeyBinding { key: "Escape".to_string(), modifiers: InputModifiers::default() },
            ],
            mouse_buttons: vec![],
        });
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a string to an egui Key
fn string_to_key(s: &str) -> Option<Key> {
    match s {
        "A" => Some(Key::A),
        "B" => Some(Key::B),
        "C" => Some(Key::C),
        "D" => Some(Key::D),
        "E" => Some(Key::E),
        "F" => Some(Key::F),
        "G" => Some(Key::G),
        "H" => Some(Key::H),
        "I" => Some(Key::I),
        "J" => Some(Key::J),
        "K" => Some(Key::K),
        "L" => Some(Key::L),
        "M" => Some(Key::M),
        "N" => Some(Key::N),
        "O" => Some(Key::O),
        "P" => Some(Key::P),
        "Q" => Some(Key::Q),
        "R" => Some(Key::R),
        "S" => Some(Key::S),
        "T" => Some(Key::T),
        "U" => Some(Key::U),
        "V" => Some(Key::V),
        "W" => Some(Key::W),
        "X" => Some(Key::X),
        "Y" => Some(Key::Y),
        "Z" => Some(Key::Z),
        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),
        "Space" => Some(Key::Space),
        "Enter" => Some(Key::Enter),
        "Escape" => Some(Key::Escape),
        "Tab" => Some(Key::Tab),
        "Backspace" => Some(Key::Backspace),
        "ArrowUp" => Some(Key::ArrowUp),
        "ArrowDown" => Some(Key::ArrowDown),
        "ArrowLeft" => Some(Key::ArrowLeft),
        "ArrowRight" => Some(Key::ArrowRight),
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        _ => None,
    }
}

fn string_to_mouse_button(s: &str) -> Option<PointerButton> {
    match s.to_lowercase().as_str() {
        "left" | "primary" => Some(PointerButton::Primary),
        "right" | "secondary" => Some(PointerButton::Secondary),
        "middle" => Some(PointerButton::Middle),
        _ => None,
    }
}
