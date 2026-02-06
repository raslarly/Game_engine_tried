//! Demo Player Controller
//! 
//! Example usage of the Phoenix Engine player system.

use glam::Vec2;

/// Player constants for the demo
pub mod constants {
    pub const PLAYER_WIDTH: f32 = 32.0;
    pub const PLAYER_HEIGHT: f32 = 48.0;
    pub const MOVE_SPEED: f32 = 280.0;
    pub const RUN_SPEED: f32 = 400.0;
    pub const JUMP_FORCE: f32 = 520.0;
    pub const DOUBLE_JUMP_FORCE: f32 = 420.0;
    pub const DASH_SPEED: f32 = 650.0;
    pub const DASH_DURATION: f32 = 0.2;
    pub const COYOTE_TIME: f32 = 0.12;
    pub const JUMP_BUFFER: f32 = 0.15;
    pub const MAX_FALL_SPEED: f32 = -800.0;
}

/// Demo player state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoPlayerState {
    Idle,
    Running,
    Jumping,
    Falling,
    DoubleJumping,
    Dashing,
    Attacking,
    Hurt,
}

/// Demo player controller
pub struct DemoPlayerController {
    pub state: DemoPlayerState,
    pub position: Vec2,
    pub velocity: Vec2,
    pub facing_right: bool,
    pub is_grounded: bool,
    pub can_double_jump: bool,
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
    pub dash_timer: f32,
    pub dash_cooldown: f32,
    pub attack_timer: f32,
    pub invincibility_timer: f32,
    pub health: i32,
    pub max_health: i32,
}

impl DemoPlayerController {
    pub fn new(spawn_pos: Vec2) -> Self {
        Self {
            state: DemoPlayerState::Idle,
            position: spawn_pos,
            velocity: Vec2::ZERO,
            facing_right: true,
            is_grounded: false,
            can_double_jump: true,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            dash_timer: 0.0,
            dash_cooldown: 0.0,
            attack_timer: 0.0,
            invincibility_timer: 0.0,
            health: 100,
            max_health: 100,
        }
    }
    
    /// Process horizontal movement input
    pub fn handle_horizontal_input(&mut self, input: f32, is_running: bool) {
        if self.state == DemoPlayerState::Dashing || self.state == DemoPlayerState::Hurt {
            return;
        }
        
        let speed = if is_running {
            constants::RUN_SPEED
        } else {
            constants::MOVE_SPEED
        };
        
        self.velocity.x = input * speed;
        
        if input > 0.01 {
            self.facing_right = true;
        } else if input < -0.01 {
            self.facing_right = false;
        }
    }
    
    /// Handle jump input
    pub fn handle_jump(&mut self) {
        // Buffer the jump
        self.jump_buffer_timer = constants::JUMP_BUFFER;
    }
    
    /// Try to execute a jump
    fn try_jump(&mut self) -> bool {
        // Ground jump or coyote time jump
        if self.is_grounded || self.coyote_timer > 0.0 {
            self.velocity.y = constants::JUMP_FORCE;
            self.is_grounded = false;
            self.coyote_timer = 0.0;
            self.state = DemoPlayerState::Jumping;
            return true;
        }
        
        // Double jump
        if self.can_double_jump {
            self.velocity.y = constants::DOUBLE_JUMP_FORCE;
            self.can_double_jump = false;
            self.state = DemoPlayerState::DoubleJumping;
            return true;
        }
        
        false
    }
    
    /// Handle dash input
    pub fn handle_dash(&mut self, direction: Vec2) {
        if self.dash_cooldown > 0.0 || self.state == DemoPlayerState::Dashing {
            return;
        }
        
        let dash_dir = if direction.length_squared() > 0.01 {
            direction.normalize()
        } else if self.facing_right {
            Vec2::X
        } else {
            -Vec2::X
        };
        
        self.velocity = dash_dir * constants::DASH_SPEED;
        self.dash_timer = constants::DASH_DURATION;
        self.dash_cooldown = 0.8;
        self.state = DemoPlayerState::Dashing;
    }
    
    /// Handle attack input
    pub fn handle_attack(&mut self) {
        if self.attack_timer > 0.0 || self.state == DemoPlayerState::Dashing {
            return;
        }
        
        self.attack_timer = 0.4;
        self.state = DemoPlayerState::Attacking;
    }
    
    /// Update player state
    pub fn update(&mut self, delta: f32, gravity: f32) {
        // Update timers
        self.coyote_timer -= delta;
        self.jump_buffer_timer -= delta;
        self.dash_timer -= delta;
        self.dash_cooldown -= delta;
        self.attack_timer -= delta;
        self.invincibility_timer -= delta;
        
        // Check jump buffer
        if self.jump_buffer_timer > 0.0 {
            if self.try_jump() {
                self.jump_buffer_timer = 0.0;
            }
        }
        
        // End dash
        if self.state == DemoPlayerState::Dashing && self.dash_timer <= 0.0 {
            self.state = DemoPlayerState::Falling;
        }
        
        // End attack
        if self.state == DemoPlayerState::Attacking && self.attack_timer <= 0.0 {
            if self.is_grounded {
                self.state = DemoPlayerState::Idle;
            } else {
                self.state = DemoPlayerState::Falling;
            }
        }
        
        // Apply gravity (except during dash)
        if self.state != DemoPlayerState::Dashing {
            self.velocity.y -= gravity * delta;
            self.velocity.y = self.velocity.y.max(constants::MAX_FALL_SPEED);
        }
        
        // Update position
        self.position += self.velocity * delta;
        
        // Update state based on velocity
        if self.state != DemoPlayerState::Dashing 
            && self.state != DemoPlayerState::Attacking 
            && self.state != DemoPlayerState::Hurt 
        {
            if self.is_grounded {
                if self.velocity.x.abs() > 1.0 {
                    self.state = DemoPlayerState::Running;
                } else {
                    self.state = DemoPlayerState::Idle;
                }
            } else if self.velocity.y > 0.0 {
                if self.can_double_jump {
                    self.state = DemoPlayerState::Jumping;
                } else {
                    self.state = DemoPlayerState::DoubleJumping;
                }
            } else {
                self.state = DemoPlayerState::Falling;
            }
        }
    }
    
    /// Set grounded state (called after physics/collision update)
    pub fn set_grounded(&mut self, grounded: bool) {
        let was_grounded = self.is_grounded;
        self.is_grounded = grounded;
        
        if grounded {
            self.can_double_jump = true;
            self.velocity.y = 0.0;
        } else if was_grounded {
            // Just left ground, start coyote time
            self.coyote_timer = constants::COYOTE_TIME;
        }
    }
    
    /// Take damage
    pub fn take_damage(&mut self, damage: i32, knockback_dir: Vec2) -> bool {
        if self.invincibility_timer > 0.0 {
            return false;
        }
        
        self.health -= damage;
        self.invincibility_timer = 1.0;
        self.state = DemoPlayerState::Hurt;
        self.velocity = knockback_dir * 300.0;
        
        self.health > 0
    }
    
    /// Heal the player
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }
    
    /// Get the attack hitbox position (if attacking)
    pub fn get_attack_hitbox(&self) -> Option<(Vec2, f32, f32)> {
        if self.state != DemoPlayerState::Attacking {
            return None;
        }
        
        let offset = if self.facing_right { 40.0 } else { -40.0 };
        let hitbox_pos = self.position + Vec2::new(offset, 0.0);
        
        Some((hitbox_pos, 50.0, 40.0)) // position, width, height
    }
    
    /// Get collision bounds
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        let half = Vec2::new(constants::PLAYER_WIDTH / 2.0, constants::PLAYER_HEIGHT / 2.0);
        (self.position - half, self.position + half)
    }
}
