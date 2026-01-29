//! Scene Management
//! 
//! Handles scene loading, saving, and entity serialization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use glam::Vec2;

/// Scene data that can be saved and loaded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub entities: Vec<SerializedEntity>,
    pub metadata: SceneMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    pub version: u32,
    pub author: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEntity {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub layer: i32,
    pub components: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedTransform {
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
    pub z_order: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedSprite {
    pub texture_id: String,
    pub color: [f32; 4],
    pub flip_x: bool,
    pub flip_y: bool,
    pub pivot: [f32; 2],
    pub visible: bool,
}

impl Default for SceneMetadata {
    fn default() -> Self {
        Self {
            version: 1,
            author: String::new(),
            description: String::new(),
        }
    }
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entities: Vec::new(),
            metadata: SceneMetadata::default(),
        }
    }
    
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read scene file: {}", e))?;
        
        let scene: Scene = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse scene: {}", e))?;
        
        Ok(scene)
    }
    
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize scene: {}", e))?;
        
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write scene file: {}", e))?;
        
        Ok(())
    }
    
    pub fn add_entity(&mut self, entity: SerializedEntity) {
        self.entities.push(entity);
    }
    
    pub fn remove_entity(&mut self, id: &str) {
        self.entities.retain(|e| e.id != id);
    }
    
    pub fn get_entity(&self, id: &str) -> Option<&SerializedEntity> {
        self.entities.iter().find(|e| e.id == id)
    }
    
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut SerializedEntity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }
}

impl SerializedEntity {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            active: true,
            layer: 0,
            components: HashMap::new(),
        }
    }
    
    pub fn with_transform(mut self, position: Vec2, rotation: f32, scale: Vec2) -> Self {
        let transform = SerializedTransform {
            position: [position.x, position.y],
            rotation,
            scale: [scale.x, scale.y],
            z_order: 0.0,
        };
        self.components.insert(
            "Transform".to_string(),
            serde_json::to_value(transform).unwrap(),
        );
        self
    }
    
    pub fn with_sprite(mut self, texture_id: &str) -> Self {
        let sprite = SerializedSprite {
            texture_id: texture_id.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            flip_x: false,
            flip_y: false,
            pivot: [0.5, 0.5],
            visible: true,
        };
        self.components.insert(
            "Sprite".to_string(),
            serde_json::to_value(sprite).unwrap(),
        );
        self
    }
}
