//! Audio Management Module
//! 
//! Handles sound effects and music playback using rodio.

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use serde::{Deserialize, Serialize};

/// Audio clip data
pub struct AudioClip {
    data: Arc<Vec<u8>>,
}

impl AudioClip {
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
        }
    }
}

/// Audio manager handles all audio playback
pub struct AudioManager {
    #[allow(dead_code)]
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    clips: HashMap<String, AudioClip>,
    music_sink: Option<Sink>,
    sfx_sinks: Vec<Sink>,
    config: AudioConfig,
}

impl AudioManager {
    /// Create a new audio manager
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio stream: {}", e))?;
        
        Ok(Self {
            stream,
            stream_handle,
            clips: HashMap::new(),
            music_sink: None,
            sfx_sinks: Vec::new(),
            config: AudioConfig::default(),
        })
    }
    
    /// Load an audio clip from bytes
    pub fn load_clip(&mut self, id: &str, data: Vec<u8>) {
        self.clips.insert(id.to_string(), AudioClip::from_bytes(data));
    }
    
    /// Play a sound effect
    pub fn play_sfx(&mut self, id: &str) -> Result<(), String> {
        let clip = self.clips.get(id)
            .ok_or_else(|| format!("Audio clip '{}' not found", id))?;
        
        let cursor = Cursor::new(clip.data.as_ref().clone());
        let source = Decoder::new(cursor)
            .map_err(|e| format!("Failed to decode audio: {}", e))?;
        
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create sink: {}", e))?;
        
        sink.set_volume(self.config.master_volume * self.config.sfx_volume);
        sink.append(source);
        
        // Clean up finished sinks
        self.sfx_sinks.retain(|s| !s.empty());
        self.sfx_sinks.push(sink);
        
        Ok(())
    }
    
    /// Play a sound effect with volume
    pub fn play_sfx_with_volume(&mut self, id: &str, volume: f32) -> Result<(), String> {
        let clip = self.clips.get(id)
            .ok_or_else(|| format!("Audio clip '{}' not found", id))?;
        
        let cursor = Cursor::new(clip.data.as_ref().clone());
        let source = Decoder::new(cursor)
            .map_err(|e| format!("Failed to decode audio: {}", e))?;
        
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create sink: {}", e))?;
        
        sink.set_volume(self.config.master_volume * self.config.sfx_volume * volume);
        sink.append(source);
        
        self.sfx_sinks.retain(|s| !s.empty());
        self.sfx_sinks.push(sink);
        
        Ok(())
    }
    
    /// Play music (looping)
    pub fn play_music(&mut self, id: &str) -> Result<(), String> {
        // Stop current music
        self.stop_music();
        
        let clip = self.clips.get(id)
            .ok_or_else(|| format!("Music '{}' not found", id))?;
        
        let cursor = Cursor::new(clip.data.as_ref().clone());
        let source = Decoder::new(cursor)
            .map_err(|e| format!("Failed to decode music: {}", e))?
            .repeat_infinite();
        
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create music sink: {}", e))?;
        
        sink.set_volume(self.config.master_volume * self.config.music_volume);
        sink.append(source);
        
        self.music_sink = Some(sink);
        Ok(())
    }
    
    /// Stop music
    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
        }
    }
    
    /// Pause music
    pub fn pause_music(&self) {
        if let Some(ref sink) = self.music_sink {
            sink.pause();
        }
    }
    
    /// Resume music
    pub fn resume_music(&self) {
        if let Some(ref sink) = self.music_sink {
            sink.play();
        }
    }
    
    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f32) {
        self.config.master_volume = volume.clamp(0.0, 1.0);
        self.update_volumes();
    }
    
    /// Set music volume
    pub fn set_music_volume(&mut self, volume: f32) {
        self.config.music_volume = volume.clamp(0.0, 1.0);
        self.update_volumes();
    }
    
    /// Set SFX volume
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.config.sfx_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Update volumes for active audio
    fn update_volumes(&self) {
        if let Some(ref sink) = self.music_sink {
            sink.set_volume(self.config.master_volume * self.config.music_volume);
        }
    }
    
    /// Get the audio config
    pub fn config(&self) -> &AudioConfig {
        &self.config
    }
    
    /// Is music playing?
    pub fn is_music_playing(&self) -> bool {
        self.music_sink.as_ref().map(|s| !s.is_paused() && !s.empty()).unwrap_or(false)
    }
}

/// Audio source component for entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    pub clip_id: String,
    pub volume: f32,
    pub spatial: bool,
    pub max_distance: f32,
    pub play_on_start: bool,
}

impl Default for AudioSource {
    fn default() -> Self {
        Self {
            clip_id: String::new(),
            volume: 1.0,
            spatial: false,
            max_distance: 100.0,
            play_on_start: false,
        }
    }
}
