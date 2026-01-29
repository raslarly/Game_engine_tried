//! Time Management Module
//! 
//! Handles delta time, fixed timestep, and timing utilities.

use instant::Instant;
use serde::{Deserialize, Serialize};

/// Time management for the game loop
pub struct Time {
    /// Time when the application started
    start_time: Instant,
    /// Time at the start of the current frame
    frame_start: Instant,
    /// Time elapsed since last frame (seconds)
    delta_time: f32,
    /// Unscaled delta time
    unscaled_delta_time: f32,
    /// Time scale for slow-mo effects
    time_scale: f32,
    /// Fixed timestep for physics (seconds)
    fixed_timestep: f32,
    /// Accumulated time for fixed updates
    fixed_accumulator: f32,
    /// Total elapsed time since start (seconds)
    elapsed: f32,
    /// Frame count
    frame_count: u64,
    /// FPS calculation
    fps: f32,
    fps_frame_count: u32,
    fps_time_accumulator: f32,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            frame_start: now,
            delta_time: 0.0,
            unscaled_delta_time: 0.0,
            time_scale: 1.0,
            fixed_timestep: 1.0 / 60.0,
            fixed_accumulator: 0.0,
            elapsed: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_frame_count: 0,
            fps_time_accumulator: 0.0,
        }
    }
    
    /// Call at the start of each frame
    pub fn update(&mut self) {
        let now = Instant::now();
        let raw_delta = now.duration_since(self.frame_start).as_secs_f32();
        
        // Clamp delta time to prevent spiral of death
        self.unscaled_delta_time = raw_delta.min(0.1);
        self.delta_time = self.unscaled_delta_time * self.time_scale;
        
        self.frame_start = now;
        self.elapsed = now.duration_since(self.start_time).as_secs_f32();
        self.frame_count += 1;
        
        // Accumulate for fixed timestep
        self.fixed_accumulator += self.delta_time;
        
        // FPS calculation
        self.fps_frame_count += 1;
        self.fps_time_accumulator += self.unscaled_delta_time;
        if self.fps_time_accumulator >= 1.0 {
            self.fps = self.fps_frame_count as f32 / self.fps_time_accumulator;
            self.fps_frame_count = 0;
            self.fps_time_accumulator = 0.0;
        }
    }
    
    /// Check if a fixed update should run, and consume time if so
    pub fn should_fixed_update(&mut self) -> bool {
        if self.fixed_accumulator >= self.fixed_timestep {
            self.fixed_accumulator -= self.fixed_timestep;
            true
        } else {
            false
        }
    }
    
    /// Delta time (affected by time scale)
    pub fn delta(&self) -> f32 {
        self.delta_time
    }
    
    /// Unscaled delta time
    pub fn unscaled_delta(&self) -> f32 {
        self.unscaled_delta_time
    }
    
    /// Time scale
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }
    
    /// Set time scale (for slow-mo effects)
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }
    
    /// Fixed timestep value
    pub fn fixed_delta(&self) -> f32 {
        self.fixed_timestep
    }
    
    /// Set fixed timestep
    pub fn set_fixed_timestep(&mut self, timestep: f32) {
        self.fixed_timestep = timestep.max(0.001);
    }
    
    /// Total elapsed time since start
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }
    
    /// Current frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
    
    /// Current FPS
    pub fn fps(&self) -> f32 {
        self.fps
    }
    
    /// Interpolation alpha for rendering between fixed updates
    pub fn fixed_alpha(&self) -> f32 {
        (self.fixed_accumulator / self.fixed_timestep).min(1.0)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer utility for delays and cooldowns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    duration: f32,
    elapsed: f32,
    repeating: bool,
    paused: bool,
    finished: bool,
}

impl Timer {
    pub fn new(duration: f32, repeating: bool) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            repeating,
            paused: false,
            finished: false,
        }
    }
    
    pub fn once(duration: f32) -> Self {
        Self::new(duration, false)
    }
    
    pub fn repeating(duration: f32) -> Self {
        Self::new(duration, true)
    }
    
    pub fn update(&mut self, delta: f32) -> bool {
        if self.paused || self.finished {
            return false;
        }
        
        self.elapsed += delta;
        
        if self.elapsed >= self.duration {
            if self.repeating {
                self.elapsed -= self.duration;
            } else {
                self.finished = true;
            }
            true
        } else {
            false
        }
    }
    
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
    }
    
    pub fn pause(&mut self) {
        self.paused = true;
    }
    
    pub fn resume(&mut self) {
        self.paused = false;
    }
    
    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).min(1.0)
    }
    
    pub fn remaining(&self) -> f32 {
        (self.duration - self.elapsed).max(0.0)
    }
    
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
