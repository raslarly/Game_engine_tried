//! Renderer Module
//! 
//! Handles 2D sprite rendering and drawing operations.

use egui::{Color32, Pos2, Rect, Vec2 as EguiVec2};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sprite component for rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    /// Texture identifier
    pub texture_id: String,
    /// Source rectangle in the texture (for sprite sheets)
    pub source_rect: Option<SpriteRect>,
    /// Tint color
    pub color: [f32; 4],
    /// Flip horizontally
    pub flip_x: bool,
    /// Flip vertically
    pub flip_y: bool,
    /// Pivot point (0-1, where 0.5, 0.5 is center)
    pub pivot: [f32; 2],
    /// Is visible
    pub visible: bool,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture_id: String::new(),
            source_rect: None,
            color: [1.0, 1.0, 1.0, 1.0],
            flip_x: false,
            flip_y: false,
            pivot: [0.5, 0.5],
            visible: true,
        }
    }
}

impl Sprite {
    pub fn new(texture_id: &str) -> Self {
        Self {
            texture_id: texture_id.to_string(),
            ..Default::default()
        }
    }
    
    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = [r, g, b, a];
        self
    }
    
    pub fn with_source_rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.source_rect = Some(SpriteRect { x, y, width: w, height: h });
        self
    }
    
    pub fn flipped_x(mut self) -> Self {
        self.flip_x = true;
        self
    }
    
    pub fn flipped_y(mut self) -> Self {
        self.flip_y = true;
        self
    }
}

/// Rectangle for sprite source regions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpriteRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl SpriteRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

/// Animation component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// List of frame indices or texture IDs
    pub frames: Vec<AnimationFrame>,
    /// Current frame index
    pub current_frame: usize,
    /// Time per frame in seconds
    pub frame_time: f32,
    /// Accumulated time
    pub elapsed: f32,
    /// Is looping
    pub looping: bool,
    /// Is playing
    pub playing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub texture_id: String,
    pub source_rect: Option<SpriteRect>,
    pub duration: Option<f32>, // Override default frame_time
}

impl Animation {
    pub fn new(frames: Vec<AnimationFrame>, frame_time: f32) -> Self {
        Self {
            frames,
            current_frame: 0,
            frame_time,
            elapsed: 0.0,
            looping: true,
            playing: true,
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        if !self.playing || self.frames.is_empty() {
            return;
        }
        
        self.elapsed += delta;
        
        let current_duration = self.frames[self.current_frame]
            .duration
            .unwrap_or(self.frame_time);
        
        if self.elapsed >= current_duration {
            self.elapsed -= current_duration;
            self.current_frame += 1;
            
            if self.current_frame >= self.frames.len() {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    self.playing = false;
                }
            }
        }
    }
    
    pub fn current_frame_data(&self) -> Option<&AnimationFrame> {
        self.frames.get(self.current_frame)
    }
    
    pub fn play(&mut self) {
        self.playing = true;
    }
    
    pub fn pause(&mut self) {
        self.playing = false;
    }
    
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed = 0.0;
        self.playing = true;
    }
}

/// Camera for 2D view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera2D {
    /// Camera position (center of view)
    pub position: Vec2,
    /// Zoom level (1.0 = normal)
    pub zoom: f32,
    /// Rotation in radians
    pub rotation: f32,
    /// Viewport size
    pub viewport_size: Vec2,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(1920.0, 1080.0),
        }
    }
}

impl Camera2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            viewport_size: Vec2::new(width, height),
            ..Default::default()
        }
    }
    
    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let relative = world_pos - self.position;
        let scaled = relative * self.zoom;
        
        Vec2::new(
            scaled.x + self.viewport_size.x / 2.0,
            self.viewport_size.y / 2.0 - scaled.y, // Flip Y for screen coords
        )
    }
    
    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let centered = Vec2::new(
            screen_pos.x - self.viewport_size.x / 2.0,
            self.viewport_size.y / 2.0 - screen_pos.y,
        );
        let unscaled = centered / self.zoom;
        unscaled + self.position
    }
    
    /// Get the visible world bounds
    pub fn visible_bounds(&self) -> (Vec2, Vec2) {
        let half_size = self.viewport_size / (2.0 * self.zoom);
        let min = self.position - half_size;
        let max = self.position + half_size;
        (min, max)
    }
    
    /// Smoothly follow a target position
    pub fn follow(&mut self, target: Vec2, smoothness: f32, delta: f32) {
        let t = 1.0 - (-smoothness * delta).exp();
        self.position = self.position.lerp(target, t);
    }
}

/// The main renderer struct
pub struct Renderer {
    /// Loaded textures
    textures: HashMap<String, egui::TextureHandle>,
    /// Current camera
    pub camera: Camera2D,
    /// Background color
    pub clear_color: Color32,
    /// Debug drawing enabled
    pub debug_draw: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            camera: Camera2D::default(),
            clear_color: Color32::from_rgb(20, 20, 25),
            debug_draw: false,
        }
    }
    
    /// Load a texture from bytes
    pub fn load_texture(
        &mut self,
        ctx: &egui::Context,
        id: &str,
        image_data: &[u8],
    ) -> Result<(), String> {
        let image = image::load_from_memory(image_data)
            .map_err(|e| format!("Failed to load image: {}", e))?;
        
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture = ctx.load_texture(id, color_image, egui::TextureOptions::NEAREST);
        
        self.textures.insert(id.to_string(), texture);
        Ok(())
    }
    
    /// Get a texture by ID
    pub fn get_texture(&self, id: &str) -> Option<&egui::TextureHandle> {
        self.textures.get(id)
    }
    
    /// Draw a sprite
    pub fn draw_sprite(
        &self,
        painter: &egui::Painter,
        sprite: &Sprite,
        position: Vec2,
        scale: Vec2,
        rotation: f32,
    ) {
        if !sprite.visible {
            return;
        }
        
        if let Some(texture) = self.textures.get(&sprite.texture_id) {
            let screen_pos = self.camera.world_to_screen(position);
            let size = texture.size_vec2() * EguiVec2::new(scale.x, scale.y) * self.camera.zoom;
            
            let pivot_offset = EguiVec2::new(
                size.x * sprite.pivot[0],
                size.y * sprite.pivot[1],
            );
            
            let rect = Rect::from_min_size(
                Pos2::new(screen_pos.x - pivot_offset.x, screen_pos.y - pivot_offset.y),
                size,
            );
            
            let uv = if let Some(source) = &sprite.source_rect {
                let tex_size = texture.size_vec2();
                Rect::from_min_max(
                    Pos2::new(source.x / tex_size.x, source.y / tex_size.y),
                    Pos2::new(
                        (source.x + source.width) / tex_size.x,
                        (source.y + source.height) / tex_size.y,
                    ),
                )
            } else {
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0))
            };
            
            let color = Color32::from_rgba_unmultiplied(
                (sprite.color[0] * 255.0) as u8,
                (sprite.color[1] * 255.0) as u8,
                (sprite.color[2] * 255.0) as u8,
                (sprite.color[3] * 255.0) as u8,
            );
            
            painter.image(texture.id(), rect, uv, color);
        }
    }
    
    /// Draw a debug rectangle
    pub fn draw_debug_rect(&self, painter: &egui::Painter, min: Vec2, max: Vec2, color: Color32) {
        if !self.debug_draw {
            return;
        }
        
        let screen_min = self.camera.world_to_screen(min);
        let screen_max = self.camera.world_to_screen(max);
        
        let rect = Rect::from_two_pos(
            Pos2::new(screen_min.x, screen_max.y),
            Pos2::new(screen_max.x, screen_min.y),
        );
        
        painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, color));
    }
    
    /// Draw a debug circle
    pub fn draw_debug_circle(&self, painter: &egui::Painter, center: Vec2, radius: f32, color: Color32) {
        if !self.debug_draw {
            return;
        }
        
        let screen_center = self.camera.world_to_screen(center);
        let screen_radius = radius * self.camera.zoom;
        
        painter.circle_stroke(
            Pos2::new(screen_center.x, screen_center.y),
            screen_radius,
            egui::Stroke::new(1.0, color),
        );
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Text rendering component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text2D {
    pub text: String,
    pub font_size: f32,
    pub color: [f32; 4],
    pub anchor: TextAnchor,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum TextAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    #[default]
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for Text2D {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 16.0,
            color: [1.0, 1.0, 1.0, 1.0],
            anchor: TextAnchor::MiddleCenter,
        }
    }
}
