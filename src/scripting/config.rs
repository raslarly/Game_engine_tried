//! Configuration Management
//! 
//! Allows game configurations to be loaded from TOML, JSON, or Python scripts.

#[cfg(feature = "python")]
use super::runtime::PythonRuntime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration value types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

impl ConfigValue {
    pub fn as_int(&self) -> Option<i64> {
        match self {
            ConfigValue::Int(v) => Some(*v),
            ConfigValue::Float(v) => Some(*v as i64),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ConfigValue::Float(v) => Some(*v),
            ConfigValue::Int(v) => Some(*v as f64),
            _ => None,
        }
    }
    
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ConfigValue::String(v) => Some(v),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ConfigValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}

/// Game configuration section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub enemies: HashMap<String, EnemyConfig>,
    pub items: HashMap<String, ItemConfig>,
    pub levels: HashMap<String, LevelConfig>,
    pub custom: HashMap<String, ConfigValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub max_health: i32,
    pub move_speed: f32,
    pub jump_force: f32,
    pub attack_damage: i32,
    pub attack_cooldown: f32,
    pub invincibility_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub name: String,
    pub max_health: i32,
    pub move_speed: f32,
    pub attack_damage: i32,
    pub attack_range: f32,
    pub detection_range: f32,
    pub drop_table: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemConfig {
    pub name: String,
    pub description: String,
    pub item_type: String,
    pub value: i32,
    pub effects: HashMap<String, ConfigValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub spawn_point: [f32; 2],
    pub bounds: [[f32; 2]; 2],
    pub background_music: String,
    pub enemy_spawns: Vec<EnemySpawn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemySpawn {
    pub enemy_type: String,
    pub position: [f32; 2],
    pub respawn_time: Option<f32>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            max_health: 100,
            move_speed: 200.0,
            jump_force: 500.0,
            attack_damage: 25,
            attack_cooldown: 0.5,
            invincibility_time: 1.0,
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player: PlayerConfig::default(),
            enemies: HashMap::new(),
            items: HashMap::new(),
            levels: HashMap::new(),
            custom: HashMap::new(),
        }
    }
}

/// Configuration manager that can load configs from various sources
pub struct ConfigManager {
    config: GameConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: GameConfig::default(),
        }
    }
    
    /// Load configuration from a TOML file
    pub fn load_toml(&mut self, path: &Path) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        self.config = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        Ok(())
    }
    
    /// Load configuration from a JSON file
    pub fn load_json(&mut self, path: &Path) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        self.config = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        Ok(())
    }
    
    /// Load and apply configuration from a Python script (requires python feature)
    #[cfg(feature = "python")]
    pub fn load_python(&mut self, runtime: &PythonRuntime, script_path: &Path) -> Result<(), String> {
        let code = std::fs::read_to_string(script_path)
            .map_err(|e| format!("Failed to read script: {}", e))?;
        
        // The Python script should define a `get_config()` function that returns a dict
        let full_code = format!(
            r#"
import json

{}

_config_result = json.dumps(get_config())
"#,
            code
        );
        
        runtime.execute(&full_code)?;
        
        // Get the JSON result
        let json_str: String = runtime.eval("_config_result")?;
        
        // Parse it back into our config
        self.config = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse Python config: {}", e))?;
        
        Ok(())
    }
    
    /// Save configuration to TOML
    pub fn save_toml(&self, path: &Path) -> Result<(), String> {
        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        
        Ok(())
    }
    
    /// Save configuration to JSON
    pub fn save_json(&self, path: &Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        
        Ok(())
    }
    
    /// Get the current configuration
    pub fn config(&self) -> &GameConfig {
        &self.config
    }
    
    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut GameConfig {
        &mut self.config
    }
    
    /// Get player config
    pub fn player(&self) -> &PlayerConfig {
        &self.config.player
    }
    
    /// Get an enemy config by type
    pub fn enemy(&self, enemy_type: &str) -> Option<&EnemyConfig> {
        self.config.enemies.get(enemy_type)
    }
    
    /// Get an item config by ID
    pub fn item(&self, item_id: &str) -> Option<&ItemConfig> {
        self.config.items.get(item_id)
    }
    
    /// Get a level config by name
    pub fn level(&self, level_name: &str) -> Option<&LevelConfig> {
        self.config.levels.get(level_name)
    }
    
    /// Get a custom config value
    pub fn custom(&self, key: &str) -> Option<&ConfigValue> {
        self.config.custom.get(key)
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
