//! Resource Management Module
//! 
//! Handles loading and caching of game assets.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Texture,
    Audio,
    Font,
    Data,
    Script,
}

/// Resource handle for referencing loaded assets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceHandle {
    pub id: String,
    pub resource_type: ResourceType,
}

/// Resource manager for loading and caching assets
pub struct ResourceManager {
    /// Base path for assets
    base_path: PathBuf,
    /// Cached raw data
    raw_data: HashMap<String, Vec<u8>>,
    /// Resource metadata
    metadata: HashMap<String, ResourceMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    pub path: String,
    pub resource_type: ResourceType,
    pub size: usize,
}

impl ResourceManager {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            raw_data: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Load a file from disk
    pub fn load_file(&mut self, id: &str, relative_path: &str, resource_type: ResourceType) -> Result<(), String> {
        let full_path = self.base_path.join(relative_path);
        let data = fs::read(&full_path)
            .map_err(|e| format!("Failed to load '{}': {}", relative_path, e))?;
        
        let metadata = ResourceMetadata {
            path: relative_path.to_string(),
            resource_type,
            size: data.len(),
        };
        
        self.raw_data.insert(id.to_string(), data);
        self.metadata.insert(id.to_string(), metadata);
        
        Ok(())
    }
    
    /// Get raw data for a resource
    pub fn get_data(&self, id: &str) -> Option<&Vec<u8>> {
        self.raw_data.get(id)
    }
    
    /// Check if a resource is loaded
    pub fn is_loaded(&self, id: &str) -> bool {
        self.raw_data.contains_key(id)
    }
    
    /// Unload a resource
    pub fn unload(&mut self, id: &str) {
        self.raw_data.remove(id);
        self.metadata.remove(id);
    }
    
    /// Get all loaded resource IDs
    pub fn loaded_ids(&self) -> impl Iterator<Item = &String> {
        self.raw_data.keys()
    }
    
    /// Get metadata for a resource
    pub fn get_metadata(&self, id: &str) -> Option<&ResourceMetadata> {
        self.metadata.get(id)
    }
    
    /// Set base path
    pub fn set_base_path<P: AsRef<Path>>(&mut self, path: P) {
        self.base_path = path.as_ref().to_path_buf();
    }
    
    /// Get base path
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }
    
    /// Load all resources from a directory
    pub fn load_directory(&mut self, relative_dir: &str) -> Result<Vec<String>, String> {
        let full_path = self.base_path.join(relative_dir);
        let mut loaded = Vec::new();
        
        if !full_path.exists() {
            return Err(format!("Directory '{}' does not exist", relative_dir));
        }
        
        for entry in fs::read_dir(&full_path)
            .map_err(|e| format!("Failed to read directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_file() {
                let extension = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                
                let resource_type = match extension.to_lowercase().as_str() {
                    "png" | "jpg" | "jpeg" | "bmp" | "gif" => ResourceType::Texture,
                    "wav" | "mp3" | "ogg" | "flac" => ResourceType::Audio,
                    "ttf" | "otf" => ResourceType::Font,
                    "json" | "toml" | "yaml" => ResourceType::Data,
                    "py" => ResourceType::Script,
                    _ => continue,
                };
                
                let file_name = path.file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                let relative = path.strip_prefix(&self.base_path)
                    .map_err(|_| "Failed to get relative path")?
                    .to_string_lossy()
                    .to_string();
                
                self.load_file(file_name, &relative, resource_type)?;
                loaded.push(file_name.to_string());
            }
        }
        
        Ok(loaded)
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new("./assets")
    }
}
