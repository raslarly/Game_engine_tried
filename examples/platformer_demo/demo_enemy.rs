//! Demo Enemy AI
//! 
//! Example enemy implementation using Phoenix Engine's combat system.

use glam::Vec2;

/// Enemy type variations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoEnemyType {
    Slime,
    Skeleton,
    Bat,
}

impl DemoEnemyType {
    pub fn stats(&self) -> EnemyStats {
        match self {
            DemoEnemyType::Slime => EnemyStats {
                max_health: 30,
                damage: 10,
                move_speed: 60.0,
                detection_range: 150.0,
                attack_range: 30.0,
                attack_cooldown: 1.5,
            },
            DemoEnemyType::Skeleton => EnemyStats {
                max_health: 50,
                damage: 15,
                move_speed: 80.0,
                detection_range: 200.0,
                attack_range: 50.0,
                attack_cooldown: 1.0,
            },
            DemoEnemyType::Bat => EnemyStats {
                max_health: 20,
                damage: 8,
                move_speed: 120.0,
                detection_range: 250.0,
                attack_range: 25.0,
                attack_cooldown: 0.8,
            },
        }
    }
}

/// Stats for an enemy type
pub struct EnemyStats {
    pub max_health: i32,
    pub damage: i32,
    pub move_speed: f32,
    pub detection_range: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
}

/// Enemy AI state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoEnemyState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Hurt,
    Dead,
}

/// Demo enemy controller
pub struct DemoEnemyController {
    pub enemy_type: DemoEnemyType,
    pub state: DemoEnemyState,
    pub position: Vec2,
    pub velocity: Vec2,
    pub facing_right: bool,
    pub health: i32,
    pub max_health: i32,
    
    // AI
    pub patrol_start: Vec2,
    pub patrol_end: Vec2,
    pub moving_to_end: bool,
    pub wait_timer: f32,
    pub attack_timer: f32,
    pub hurt_timer: f32,
    
    // Stats (cached from type)
    stats: EnemyStats,
}

impl DemoEnemyController {
    pub fn new(enemy_type: DemoEnemyType, position: Vec2) -> Self {
        let stats = enemy_type.stats();
        let max_health = stats.max_health;
        
        Self {
            enemy_type,
            state: DemoEnemyState::Idle,
            position,
            velocity: Vec2::ZERO,
            facing_right: true,
            health: max_health,
            max_health,
            patrol_start: position - Vec2::new(100.0, 0.0),
            patrol_end: position + Vec2::new(100.0, 0.0),
            moving_to_end: true,
            wait_timer: 0.0,
            attack_timer: 0.0,
            hurt_timer: 0.0,
            stats,
        }
    }
    
    /// Set patrol path
    pub fn set_patrol(&mut self, start: Vec2, end: Vec2) {
        self.patrol_start = start;
        self.patrol_end = end;
        self.state = DemoEnemyState::Patrol;
    }
    
    /// Update enemy AI
    pub fn update(&mut self, delta: f32, player_pos: Vec2) {
        // Update timers
        self.wait_timer -= delta;
        self.attack_timer -= delta;
        self.hurt_timer -= delta;
        
        // Dead enemies don't update
        if self.state == DemoEnemyState::Dead {
            return;
        }
        
        // Recover from hurt state
        if self.state == DemoEnemyState::Hurt && self.hurt_timer <= 0.0 {
            self.state = DemoEnemyState::Idle;
        }
        
        // Check for player detection
        let dist_to_player = (player_pos - self.position).length();
        let can_see_player = dist_to_player <= self.stats.detection_range;
        let in_attack_range = dist_to_player <= self.stats.attack_range;
        
        // State machine
        match self.state {
            DemoEnemyState::Idle => {
                self.velocity = Vec2::ZERO;
                
                if can_see_player {
                    self.state = DemoEnemyState::Chase;
                } else if self.wait_timer <= 0.0 {
                    self.state = DemoEnemyState::Patrol;
                }
            }
            
            DemoEnemyState::Patrol => {
                let target = if self.moving_to_end {
                    self.patrol_end
                } else {
                    self.patrol_start
                };
                
                let dir = (target - self.position).normalize_or_zero();
                self.velocity = dir * self.stats.move_speed;
                self.facing_right = dir.x > 0.0;
                
                // Check if reached target
                if (target - self.position).length() < 10.0 {
                    self.moving_to_end = !self.moving_to_end;
                    self.wait_timer = 2.0;
                    self.state = DemoEnemyState::Idle;
                }
                
                // Check for player
                if can_see_player {
                    self.state = DemoEnemyState::Chase;
                }
            }
            
            DemoEnemyState::Chase => {
                if !can_see_player {
                    // Lost player, return to patrol
                    self.state = DemoEnemyState::Patrol;
                } else if in_attack_range && self.attack_timer <= 0.0 {
                    // Attack!
                    self.state = DemoEnemyState::Attack;
                    self.attack_timer = self.stats.attack_cooldown;
                } else {
                    // Move towards player
                    let dir = (player_pos - self.position).normalize_or_zero();
                    self.velocity = dir * self.stats.move_speed * 1.2; // Faster when chasing
                    self.facing_right = dir.x > 0.0;
                }
            }
            
            DemoEnemyState::Attack => {
                self.velocity = Vec2::ZERO;
                
                // Attack happens, transition back
                if self.attack_timer <= self.stats.attack_cooldown * 0.5 {
                    if can_see_player {
                        self.state = DemoEnemyState::Chase;
                    } else {
                        self.state = DemoEnemyState::Idle;
                    }
                }
            }
            
            DemoEnemyState::Hurt => {
                // Slow down from knockback
                self.velocity *= 0.9;
            }
            
            DemoEnemyState::Dead => {}
        }
        
        // Apply velocity
        self.position += self.velocity * delta;
    }
    
    /// Take damage
    pub fn take_damage(&mut self, damage: i32, knockback: Vec2) -> bool {
        if self.state == DemoEnemyState::Dead {
            return false;
        }
        
        self.health -= damage;
        
        if self.health <= 0 {
            self.health = 0;
            self.state = DemoEnemyState::Dead;
            self.velocity = Vec2::ZERO;
            return true; // Died
        }
        
        self.state = DemoEnemyState::Hurt;
        self.hurt_timer = 0.3;
        self.velocity = knockback;
        false // Survived
    }
    
    /// Check if this enemy is currently attacking (for damage dealing)
    pub fn is_attacking(&self) -> bool {
        self.state == DemoEnemyState::Attack && 
        self.attack_timer > self.stats.attack_cooldown * 0.3 &&
        self.attack_timer < self.stats.attack_cooldown * 0.7
    }
    
    /// Get damage for this enemy type
    pub fn get_damage(&self) -> i32 {
        self.stats.damage
    }
    
    /// Get collision bounds
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        let half = Vec2::new(16.0, 16.0);
        (self.position - half, self.position + half)
    }
    
    /// Is this enemy alive?
    pub fn is_alive(&self) -> bool {
        self.state != DemoEnemyState::Dead
    }
}
