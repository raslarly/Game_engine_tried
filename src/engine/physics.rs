//! Physics Module
//! 
//! Simple 2D physics for platformers and action games.

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half = size / 2.0;
        Self {
            min: center - half,
            max: center + half,
        }
    }
    
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }
    
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }
    
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }
    
    pub fn intersects(&self, other: &AABB) -> bool {
        self.max.x >= other.min.x && self.min.x <= other.max.x &&
        self.max.y >= other.min.y && self.min.y <= other.max.y
    }
    
    pub fn translate(&self, offset: Vec2) -> AABB {
        AABB {
            min: self.min + offset,
            max: self.max + offset,
        }
    }
}

/// Circle collider
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }
    
    pub fn contains_point(&self, point: Vec2) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }
    
    pub fn intersects_circle(&self, other: &Circle) -> bool {
        let dist_sq = (other.center - self.center).length_squared();
        let radius_sum = self.radius + other.radius;
        dist_sq <= radius_sum * radius_sum
    }
    
    pub fn intersects_aabb(&self, aabb: &AABB) -> bool {
        let closest = Vec2::new(
            self.center.x.clamp(aabb.min.x, aabb.max.x),
            self.center.y.clamp(aabb.min.y, aabb.max.y),
        );
        (closest - self.center).length_squared() <= self.radius * self.radius
    }
}

/// Collider component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Collider {
    Box { width: f32, height: f32, offset: Vec2 },
    Circle { radius: f32, offset: Vec2 },
}

impl Collider {
    pub fn box_collider(width: f32, height: f32) -> Self {
        Collider::Box { width, height, offset: Vec2::ZERO }
    }
    
    pub fn circle_collider(radius: f32) -> Self {
        Collider::Circle { radius, offset: Vec2::ZERO }
    }
    
    pub fn with_offset(self, offset: Vec2) -> Self {
        match self {
            Collider::Box { width, height, .. } => 
                Collider::Box { width, height, offset },
            Collider::Circle { radius, .. } => 
                Collider::Circle { radius, offset },
        }
    }
    
    pub fn get_aabb(&self, position: Vec2) -> AABB {
        match self {
            Collider::Box { width, height, offset } => {
                let center = position + *offset;
                AABB::from_center_size(center, Vec2::new(*width, *height))
            }
            Collider::Circle { radius, offset } => {
                let center = position + *offset;
                AABB::from_center_size(center, Vec2::new(*radius * 2.0, *radius * 2.0))
            }
        }
    }
}

/// Rigidbody for physics simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rigidbody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub gravity_scale: f32,
    pub drag: f32,
    pub angular_velocity: f32,
    pub angular_drag: f32,
    pub is_kinematic: bool,
    pub freeze_rotation: bool,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            gravity_scale: 1.0,
            drag: 0.0,
            angular_velocity: 0.0,
            angular_drag: 0.0,
            is_kinematic: false,
            freeze_rotation: false,
        }
    }
}

impl Rigidbody {
    pub fn kinematic() -> Self {
        Self {
            is_kinematic: true,
            ..Default::default()
        }
    }
    
    pub fn apply_force(&mut self, force: Vec2) {
        if !self.is_kinematic {
            self.acceleration += force / self.mass;
        }
    }
    
    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if !self.is_kinematic {
            self.velocity += impulse / self.mass;
        }
    }
}

/// Physics world configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub velocity_iterations: u32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -980.0),
            velocity_iterations: 8,
        }
    }
}

/// Physics world for simulation
pub struct PhysicsWorld {
    pub config: PhysicsConfig,
}

impl PhysicsWorld {
    pub fn new(config: PhysicsConfig) -> Self {
        Self { config }
    }
    
    /// Update a rigidbody (call in fixed update)
    pub fn update_body(&self, rb: &mut Rigidbody, position: &mut Vec2, rotation: &mut f32, dt: f32) {
        if rb.is_kinematic {
            return;
        }
        
        // Apply gravity
        rb.acceleration += self.config.gravity * rb.gravity_scale;
        
        // Integrate velocity
        rb.velocity += rb.acceleration * dt;
        
        // Apply drag
        rb.velocity *= 1.0 - rb.drag * dt;
        
        // Integrate position
        *position += rb.velocity * dt;
        
        // Angular motion
        if !rb.freeze_rotation {
            *rotation += rb.angular_velocity * dt;
            rb.angular_velocity *= 1.0 - rb.angular_drag * dt;
        }
        
        // Reset acceleration
        rb.acceleration = Vec2::ZERO;
    }
    
    /// Check collision between two entities
    pub fn check_collision(
        collider_a: &Collider, pos_a: Vec2,
        collider_b: &Collider, pos_b: Vec2,
    ) -> Option<CollisionInfo> {
        let aabb_a = collider_a.get_aabb(pos_a);
        let aabb_b = collider_b.get_aabb(pos_b);
        
        if !aabb_a.intersects(&aabb_b) {
            return None;
        }
        
        // Calculate penetration
        let overlap_x = (aabb_a.max.x.min(aabb_b.max.x) - aabb_a.min.x.max(aabb_b.min.x)).max(0.0);
        let overlap_y = (aabb_a.max.y.min(aabb_b.max.y) - aabb_a.min.y.max(aabb_b.min.y)).max(0.0);
        
        let (normal, depth) = if overlap_x < overlap_y {
            let nx = if aabb_a.center().x < aabb_b.center().x { -1.0 } else { 1.0 };
            (Vec2::new(nx, 0.0), overlap_x)
        } else {
            let ny = if aabb_a.center().y < aabb_b.center().y { -1.0 } else { 1.0 };
            (Vec2::new(0.0, ny), overlap_y)
        };
        
        Some(CollisionInfo {
            normal,
            depth,
            point: (aabb_a.center() + aabb_b.center()) / 2.0,
        })
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new(PhysicsConfig::default())
    }
}

/// Collision information
#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    pub normal: Vec2,
    pub depth: f32,
    pub point: Vec2,
}
