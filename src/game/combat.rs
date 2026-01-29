//! Combat System
//! 
//! Handles combat mechanics, damage, and enemy AI.

use glam::Vec2;
use serde::{Deserialize, Serialize};
use crate::engine::time::Timer;

/// Health component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub max_health: i32,
    pub current_health: i32,
    pub invincibility_time: f32,
    invincibility_timer: f32,
}

impl Health {
    pub fn new(max_health: i32) -> Self {
        Self {
            max_health,
            current_health: max_health,
            invincibility_time: 0.5,
            invincibility_timer: 0.0,
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        if self.invincibility_timer > 0.0 {
            self.invincibility_timer -= delta;
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) -> bool {
        if self.invincibility_timer > 0.0 {
            return false;
        }
        
        self.current_health = (self.current_health - damage).max(0);
        self.invincibility_timer = self.invincibility_time;
        true
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.current_health = (self.current_health + amount).min(self.max_health);
    }
    
    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }
    
    pub fn percent(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }
    
    pub fn is_invincible(&self) -> bool {
        self.invincibility_timer > 0.0
    }
}

/// Damage types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
}

/// Damage dealer component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageDealer {
    pub damage: i32,
    pub damage_type: DamageType,
    pub knockback_force: f32,
    pub hits_player: bool,
    pub hits_enemies: bool,
    pub one_shot: bool,
    pub has_hit: bool,
}

impl DamageDealer {
    pub fn player_attack(damage: i32) -> Self {
        Self {
            damage,
            damage_type: DamageType::Physical,
            knockback_force: 200.0,
            hits_player: false,
            hits_enemies: true,
            one_shot: true,
            has_hit: false,
        }
    }
    
    pub fn enemy_attack(damage: i32) -> Self {
        Self {
            damage,
            damage_type: DamageType::Physical,
            knockback_force: 150.0,
            hits_player: true,
            hits_enemies: false,
            one_shot: false,
            has_hit: false,
        }
    }
    
    pub fn projectile(damage: i32, hits_player: bool) -> Self {
        Self {
            damage,
            damage_type: DamageType::Physical,
            knockback_force: 100.0,
            hits_player,
            hits_enemies: !hits_player,
            one_shot: true,
            has_hit: false,
        }
    }
}

/// Enemy AI states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Hurt,
    Dead,
}

/// Enemy component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub state: EnemyState,
    pub enemy_type: String,
    pub facing_right: bool,
    
    // Stats
    pub max_health: i32,
    pub current_health: i32,
    pub move_speed: f32,
    pub attack_damage: i32,
    pub attack_range: f32,
    pub detection_range: f32,
    
    // AI
    pub patrol_points: Vec<Vec2>,
    pub current_patrol_index: usize,
    pub wait_timer: Timer,
    pub attack_cooldown: Timer,
    
    // Combat
    pub invincibility: Timer,
}

impl Enemy {
    pub fn new(enemy_type: &str) -> Self {
        Self {
            state: EnemyState::Idle,
            enemy_type: enemy_type.to_string(),
            facing_right: true,
            
            max_health: 50,
            current_health: 50,
            move_speed: 100.0,
            attack_damage: 10,
            attack_range: 50.0,
            detection_range: 200.0,
            
            patrol_points: Vec::new(),
            current_patrol_index: 0,
            wait_timer: Timer::once(2.0),
            attack_cooldown: Timer::once(1.5),
            
            invincibility: Timer::once(0.3),
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        self.wait_timer.update(delta);
        self.attack_cooldown.update(delta);
        self.invincibility.update(delta);
    }
    
    pub fn take_damage(&mut self, damage: i32) -> bool {
        if !self.invincibility.is_finished() || self.state == EnemyState::Dead {
            return false;
        }
        
        self.current_health -= damage;
        self.invincibility.reset();
        
        if self.current_health <= 0 {
            self.current_health = 0;
            self.state = EnemyState::Dead;
        } else {
            self.state = EnemyState::Hurt;
        }
        
        true
    }
    
    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }
    
    pub fn can_attack(&self) -> bool {
        self.attack_cooldown.is_finished() && 
        self.state != EnemyState::Hurt &&
        self.state != EnemyState::Dead
    }
    
    pub fn can_see_player(&self, enemy_pos: Vec2, player_pos: Vec2) -> bool {
        (player_pos - enemy_pos).length() <= self.detection_range
    }
    
    pub fn in_attack_range(&self, enemy_pos: Vec2, player_pos: Vec2) -> bool {
        (player_pos - enemy_pos).length() <= self.attack_range
    }
}

/// Hitbox component for combat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2,
    pub active: bool,
    pub lifetime: Option<f32>,
}

impl Hitbox {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            offset: Vec2::ZERO,
            active: true,
            lifetime: None,
        }
    }
    
    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }
    
    pub fn with_lifetime(mut self, lifetime: f32) -> Self {
        self.lifetime = Some(lifetime);
        self
    }
    
    pub fn update(&mut self, delta: f32) {
        if let Some(ref mut lifetime) = self.lifetime {
            *lifetime -= delta;
            if *lifetime <= 0.0 {
                self.active = false;
            }
        }
    }
}

/// Loot drop information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootDrop {
    pub item_id: String,
    pub quantity: u32,
    pub velocity: Vec2,
}

impl LootDrop {
    pub fn new(item_id: &str, quantity: u32) -> Self {
        Self {
            item_id: item_id.to_string(),
            quantity,
            velocity: Vec2::new(
                rand::random::<f32>() * 100.0 - 50.0,
                rand::random::<f32>() * 200.0 + 100.0,
            ),
        }
    }
}
