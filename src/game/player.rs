//! Player System
//! 
//! Player controller and state management.

use glam::Vec2;
use serde::{Deserialize, Serialize};
use crate::engine::time::Timer;

/// Player state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerState {
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Attacking,
    Dashing,
    Hurt,
    Dead,
}

/// Player component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub state: PlayerState,
    pub facing_right: bool,
    pub is_grounded: bool,
    pub can_double_jump: bool,
    pub coyote_time: f32,
    pub jump_buffer: f32,
    
    // Stats
    pub max_health: i32,
    pub current_health: i32,
    pub move_speed: f32,
    pub run_multiplier: f32,
    pub jump_force: f32,
    pub double_jump_force: f32,
    
    // Combat
    pub attack_damage: i32,
    pub attack_cooldown: Timer,
    pub invincibility: Timer,
    pub dash_cooldown: Timer,
    pub dash_duration: Timer,
    pub dash_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Idle,
            facing_right: true,
            is_grounded: false,
            can_double_jump: true,
            coyote_time: 0.0,
            jump_buffer: 0.0,
            
            max_health: 100,
            current_health: 100,
            move_speed: 200.0,
            run_multiplier: 1.5,
            jump_force: 500.0,
            double_jump_force: 400.0,
            
            attack_damage: 25,
            attack_cooldown: Timer::once(0.5),
            invincibility: Timer::once(1.0),
            dash_cooldown: Timer::once(1.0),
            dash_duration: Timer::once(0.2),
            dash_speed: 600.0,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update player timers
    pub fn update(&mut self, delta: f32) {
        self.attack_cooldown.update(delta);
        self.invincibility.update(delta);
        self.dash_cooldown.update(delta);
        self.dash_duration.update(delta);
        
        // Update coyote time
        if !self.is_grounded {
            self.coyote_time -= delta;
        } else {
            self.coyote_time = 0.1; // 100ms coyote time
        }
        
        // Update jump buffer
        self.jump_buffer -= delta;
    }
    
    /// Can the player jump?
    pub fn can_jump(&self) -> bool {
        self.is_grounded || self.coyote_time > 0.0
    }
    
    /// Can the player attack?
    pub fn can_attack(&self) -> bool {
        self.attack_cooldown.is_finished() && 
        self.state != PlayerState::Attacking &&
        self.state != PlayerState::Dashing &&
        self.state != PlayerState::Hurt
    }
    
    /// Can the player dash?
    pub fn can_dash(&self) -> bool {
        self.dash_cooldown.is_finished() &&
        self.state != PlayerState::Attacking &&
        self.state != PlayerState::Dashing &&
        self.state != PlayerState::Hurt
    }
    
    /// Is the player invincible?
    pub fn is_invincible(&self) -> bool {
        !self.invincibility.is_finished()
    }
    
    /// Take damage
    pub fn take_damage(&mut self, damage: i32) -> bool {
        if self.is_invincible() || self.state == PlayerState::Dead {
            return false;
        }
        
        self.current_health -= damage;
        
        if self.current_health <= 0 {
            self.current_health = 0;
            self.state = PlayerState::Dead;
        } else {
            self.state = PlayerState::Hurt;
            self.invincibility.reset();
        }
        
        true
    }
    
    /// Heal the player
    pub fn heal(&mut self, amount: i32) {
        self.current_health = (self.current_health + amount).min(self.max_health);
    }
    
    /// Is the player alive?
    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }
    
    /// Get health percentage
    pub fn health_percent(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }
    
    /// Start attacking
    pub fn start_attack(&mut self) {
        if self.can_attack() {
            self.state = PlayerState::Attacking;
            self.attack_cooldown.reset();
        }
    }
    
    /// Start dashing
    pub fn start_dash(&mut self, direction: Vec2) -> Option<Vec2> {
        if self.can_dash() {
            self.state = PlayerState::Dashing;
            self.dash_cooldown.reset();
            self.dash_duration.reset();
            
            let dash_dir = if direction.length_squared() > 0.01 {
                direction.normalize()
            } else if self.facing_right {
                Vec2::X
            } else {
                -Vec2::X
            };
            
            Some(dash_dir * self.dash_speed)
        } else {
            None
        }
    }
    
    /// Buffer a jump input
    pub fn buffer_jump(&mut self) {
        self.jump_buffer = 0.15; // 150ms buffer
    }
    
    /// Check and consume jump buffer
    pub fn consume_jump_buffer(&mut self) -> bool {
        if self.jump_buffer > 0.0 && self.can_jump() {
            self.jump_buffer = 0.0;
            true
        } else {
            false
        }
    }
}

/// Controller for player movement
pub struct PlayerController;

impl PlayerController {
    /// Calculate movement velocity from input
    pub fn calculate_movement(
        player: &Player,
        horizontal_input: f32,
        is_running: bool,
    ) -> Vec2 {
        if !player.is_alive() || player.state == PlayerState::Dashing {
            return Vec2::ZERO;
        }
        
        let speed = if is_running {
            player.move_speed * player.run_multiplier
        } else {
            player.move_speed
        };
        
        Vec2::new(horizontal_input * speed, 0.0)
    }
    
    /// Calculate jump velocity
    pub fn calculate_jump(player: &mut Player, is_grounded: bool) -> Option<f32> {
        if is_grounded || player.coyote_time > 0.0 {
            player.coyote_time = 0.0;
            player.can_double_jump = true;
            return Some(player.jump_force);
        } else if player.can_double_jump {
            player.can_double_jump = false;
            return Some(player.double_jump_force);
        }
        None
    }
}
