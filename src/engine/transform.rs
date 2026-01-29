//! Transform Component
//! 
//! Handles position, rotation, and scale for game entities.

use glam::{Mat3, Vec2};
use serde::{Deserialize, Serialize};

/// Transform component for 2D positioning
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    /// Position in world space
    pub position: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// Scale factor
    pub scale: Vec2,
    /// Z-order for rendering (higher = in front)
    pub z_order: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            z_order: 0.0,
        }
    }
}

impl Transform {
    /// Create a new transform at the given position
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..Default::default()
        }
    }
    
    /// Create a transform from position vector
    pub fn from_position(position: Vec2) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
    
    /// Create a transform with position and scale
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
    
    /// Create a transform with rotation
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
    
    /// Create a transform with z-order
    pub fn with_z_order(mut self, z_order: f32) -> Self {
        self.z_order = z_order;
        self
    }
    
    /// Get the transformation matrix
    pub fn matrix(&self) -> Mat3 {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        
        Mat3::from_cols(
            glam::Vec3::new(cos_r * self.scale.x, sin_r * self.scale.x, 0.0),
            glam::Vec3::new(-sin_r * self.scale.y, cos_r * self.scale.y, 0.0),
            glam::Vec3::new(self.position.x, self.position.y, 1.0),
        )
    }
    
    /// Transform a local point to world space
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        
        let scaled = point * self.scale;
        let rotated = Vec2::new(
            scaled.x * cos_r - scaled.y * sin_r,
            scaled.x * sin_r + scaled.y * cos_r,
        );
        
        rotated + self.position
    }
    
    /// Get the forward direction (for rotation)
    pub fn forward(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }
    
    /// Get the right direction
    pub fn right(&self) -> Vec2 {
        let forward = self.forward();
        Vec2::new(forward.y, -forward.x)
    }
    
    /// Translate by an offset
    pub fn translate(&mut self, offset: Vec2) {
        self.position += offset;
    }
    
    /// Rotate by an angle in radians
    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
        // Normalize to [0, 2π)
        self.rotation = self.rotation.rem_euclid(std::f32::consts::TAU);
    }
    
    /// Look at a target position
    pub fn look_at(&mut self, target: Vec2) {
        let direction = target - self.position;
        if direction.length_squared() > 0.0001 {
            self.rotation = direction.y.atan2(direction.x);
        }
    }
    
    /// Lerp between two transforms
    pub fn lerp(&self, other: &Transform, t: f32) -> Transform {
        Transform {
            position: self.position.lerp(other.position, t),
            rotation: lerp_angle(self.rotation, other.rotation, t),
            scale: self.scale.lerp(other.scale, t),
            z_order: self.z_order + (other.z_order - self.z_order) * t,
        }
    }
}

/// Linearly interpolate between two angles (handling wrap-around)
fn lerp_angle(from: f32, to: f32, t: f32) -> f32 {
    use std::f32::consts::PI;
    
    let mut diff = to - from;
    
    // Normalize the difference to [-π, π]
    while diff > PI {
        diff -= 2.0 * PI;
    }
    while diff < -PI {
        diff += 2.0 * PI;
    }
    
    from + diff * t
}

/// Velocity component for physics
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Velocity {
    pub linear: Vec2,
    pub angular: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self {
            linear: Vec2::new(vx, vy),
            angular: 0.0,
        }
    }
    
    pub fn from_linear(linear: Vec2) -> Self {
        Self {
            linear,
            angular: 0.0,
        }
    }
    
    pub fn with_angular(mut self, angular: f32) -> Self {
        self.angular = angular;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transform_default() {
        let t = Transform::default();
        assert_eq!(t.position, Vec2::ZERO);
        assert_eq!(t.rotation, 0.0);
        assert_eq!(t.scale, Vec2::ONE);
    }
    
    #[test]
    fn test_transform_point() {
        let mut t = Transform::new(10.0, 5.0);
        t.scale = Vec2::new(2.0, 2.0);
        
        let local = Vec2::new(1.0, 0.0);
        let world = t.transform_point(local);
        
        assert!((world.x - 12.0).abs() < 0.001);
        assert!((world.y - 5.0).abs() < 0.001);
    }
}
