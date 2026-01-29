//! Project Management
//! 
//! Handles project loading, saving, and configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub settings: ProjectSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub window: WindowSettings,
    pub physics: PhysicsSettings,
    pub audio: AudioSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
    pub resizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSettings {
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub fixed_timestep: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            title: "Phoenix Game".to_string(),
            fullscreen: false,
            vsync: true,
            resizable: true,
        }
    }
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity_x: 0.0,
            gravity_y: -980.0,
            fixed_timestep: 1.0 / 60.0,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
        }
    }
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            window: WindowSettings::default(),
            physics: PhysicsSettings::default(),
            audio: AudioSettings::default(),
        }
    }
}

impl Project {
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            path,
            settings: ProjectSettings::default(),
        }
    }
    
    pub fn load(path: PathBuf) -> Result<Self, String> {
        let config_path = path.join("project.toml");
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read project file: {}", e))?;
        
        let project: Project = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse project file: {}", e))?;
        
        Ok(project)
    }
    
    pub fn save(&self) -> Result<(), String> {
        let config_path = self.path.join("project.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize project: {}", e))?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| format!("Failed to write project file: {}", e))?;
        
        Ok(())
    }
    
    pub fn create_directory_structure(&self) -> Result<(), String> {
        let dirs = ["assets", "assets/sprites", "assets/audio", "assets/fonts", "scripts", "scenes", "config"];
        
        for dir in &dirs {
            let path = self.path.join(dir);
            std::fs::create_dir_all(&path)
                .map_err(|e| format!("Failed to create directory '{}': {}", dir, e))?;
        }
        
        Ok(())
    }
}
