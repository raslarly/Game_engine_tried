//! Demo Game
//! 
//! Main game struct demonstrating Phoenix Engine usage.

use glam::Vec2;

/// Demo game configuration
pub struct DemoGameConfig {
    pub screen_width: f32,
    pub screen_height: f32,
    pub gravity: f32,
    pub player_start_pos: Vec2,
}

impl Default for DemoGameConfig {
    fn default() -> Self {
        Self {
            screen_width: 1280.0,
            screen_height: 720.0,
            gravity: 980.0,
            player_start_pos: Vec2::new(100.0, 300.0),
        }
    }
}

/// The main demo game struct
/// 
/// # Example Usage
/// 
/// ```rust
/// use phoenix_engine::examples::platformer_demo::DemoGame;
/// 
/// fn main() {
///     let mut game = DemoGame::new();
///     game.setup();
///     
///     // In your game loop:
///     // game.update(delta_time);
///     // game.render(&renderer);
/// }
/// ```
pub struct DemoGame {
    pub config: DemoGameConfig,
    pub player_pos: Vec2,
    pub player_vel: Vec2,
    pub player_health: i32,
    pub player_max_health: i32,
    pub is_grounded: bool,
    pub facing_right: bool,
    pub score: u32,
    pub coins_collected: u32,
    pub game_over: bool,
    pub level_complete: bool,
    
    // Platforms (x, y, width, height)
    pub platforms: Vec<(f32, f32, f32, f32)>,
    
    // Coins (x, y, collected)
    pub coins: Vec<(f32, f32, bool)>,
    
    // Enemies (x, y, health, facing_right)
    pub enemies: Vec<(f32, f32, i32, bool)>,
}

impl DemoGame {
    /// Create a new demo game with default configuration
    pub fn new() -> Self {
        Self::with_config(DemoGameConfig::default())
    }
    
    /// Create a new demo game with custom configuration
    pub fn with_config(config: DemoGameConfig) -> Self {
        Self {
            player_pos: config.player_start_pos,
            player_vel: Vec2::ZERO,
            player_health: 100,
            player_max_health: 100,
            is_grounded: false,
            facing_right: true,
            score: 0,
            coins_collected: 0,
            game_over: false,
            level_complete: false,
            platforms: Vec::new(),
            coins: Vec::new(),
            enemies: Vec::new(),
            config,
        }
    }
    
    /// Setup the demo level
    pub fn setup(&mut self) {
        // Create ground
        self.platforms.push((0.0, 50.0, 1280.0, 50.0));
        
        // Create floating platforms
        self.platforms.push((200.0, 150.0, 150.0, 20.0));
        self.platforms.push((400.0, 250.0, 150.0, 20.0));
        self.platforms.push((600.0, 350.0, 150.0, 20.0));
        self.platforms.push((800.0, 250.0, 150.0, 20.0));
        self.platforms.push((1000.0, 150.0, 150.0, 20.0));
        
        // Create coins
        self.coins.push((250.0, 200.0, false));
        self.coins.push((450.0, 300.0, false));
        self.coins.push((650.0, 400.0, false));
        self.coins.push((850.0, 300.0, false));
        self.coins.push((1050.0, 200.0, false));
        
        // Create enemies
        self.enemies.push((350.0, 100.0, 50, true));
        self.enemies.push((700.0, 400.0, 50, false));
    }
    
    /// Update game logic
    /// 
    /// # Arguments
    /// * `delta` - Time since last frame in seconds
    /// * `input_horizontal` - Horizontal input (-1.0 to 1.0)
    /// * `jump_pressed` - Whether jump button was pressed this frame
    /// * `attack_pressed` - Whether attack button was pressed this frame
    pub fn update(
        &mut self, 
        delta: f32, 
        input_horizontal: f32, 
        jump_pressed: bool,
        attack_pressed: bool,
    ) {
        if self.game_over || self.level_complete {
            return;
        }
        
        // Apply gravity
        if !self.is_grounded {
            self.player_vel.y -= self.config.gravity * delta;
        }
        
        // Horizontal movement
        let move_speed = 300.0;
        self.player_vel.x = input_horizontal * move_speed;
        
        if input_horizontal > 0.0 {
            self.facing_right = true;
        } else if input_horizontal < 0.0 {
            self.facing_right = false;
        }
        
        // Jump
        if jump_pressed && self.is_grounded {
            self.player_vel.y = 500.0;
            self.is_grounded = false;
        }
        
        // Apply velocity
        self.player_pos += self.player_vel * delta;
        
        // Ground collision
        self.is_grounded = false;
        for (px, py, pw, ph) in &self.platforms {
            // Simple AABB collision
            let player_bottom = self.player_pos.y - 25.0; // Assuming 50px tall player
            let platform_top = py + ph;
            
            if self.player_pos.x >= *px - 25.0 
                && self.player_pos.x <= px + pw + 25.0
                && player_bottom <= platform_top 
                && player_bottom >= *py
                && self.player_vel.y <= 0.0
            {
                self.player_pos.y = platform_top + 25.0;
                self.player_vel.y = 0.0;
                self.is_grounded = true;
            }
        }
        
        // Coin collection
        for (cx, cy, collected) in &mut self.coins {
            if !*collected {
                let dist = (Vec2::new(*cx, *cy) - self.player_pos).length();
                if dist < 30.0 {
                    *collected = true;
                    self.coins_collected += 1;
                    self.score += 100;
                }
            }
        }
        
        // Attack enemies
        if attack_pressed {
            let attack_range = 60.0;
            for (ex, ey, health, _) in &mut self.enemies {
                let dist = (Vec2::new(*ex, *ey) - self.player_pos).length();
                if dist < attack_range && *health > 0 {
                    *health -= 25;
                    if *health <= 0 {
                        self.score += 500;
                    }
                }
            }
        }
        
        // Enemy collision with player
        for (ex, ey, health, _) in &self.enemies {
            if *health > 0 {
                let dist = (Vec2::new(*ex, *ey) - self.player_pos).length();
                if dist < 40.0 {
                    self.player_health -= 10;
                    // Knockback
                    self.player_vel.y = 200.0;
                    if *ex > self.player_pos.x {
                        self.player_vel.x = -200.0;
                    } else {
                        self.player_vel.x = 200.0;
                    }
                    
                    if self.player_health <= 0 {
                        self.game_over = true;
                    }
                }
            }
        }
        
        // Check level complete (all coins collected)
        let all_collected = self.coins.iter().all(|(_, _, c)| *c);
        if all_collected {
            self.level_complete = true;
        }
        
        // Keep player in bounds
        self.player_pos.x = self.player_pos.x.clamp(25.0, self.config.screen_width - 25.0);
    }
    
    /// Get game state for rendering
    pub fn get_render_state(&self) -> DemoRenderState {
        DemoRenderState {
            player_pos: self.player_pos,
            player_facing_right: self.facing_right,
            player_health_percent: self.player_health as f32 / self.player_max_health as f32,
            platforms: self.platforms.clone(),
            coins: self.coins.iter()
                .filter(|(_, _, c)| !c)
                .map(|(x, y, _)| (*x, *y))
                .collect(),
            enemies: self.enemies.iter()
                .filter(|(_, _, h, _)| *h > 0)
                .map(|(x, y, _, f)| (*x, *y, *f))
                .collect(),
            score: self.score,
            game_over: self.game_over,
            level_complete: self.level_complete,
        }
    }
    
    /// Reset the game
    pub fn reset(&mut self) {
        self.player_pos = self.config.player_start_pos;
        self.player_vel = Vec2::ZERO;
        self.player_health = self.player_max_health;
        self.is_grounded = false;
        self.score = 0;
        self.coins_collected = 0;
        self.game_over = false;
        self.level_complete = false;
        
        // Reset coins
        for (_, _, collected) in &mut self.coins {
            *collected = false;
        }
        
        // Reset enemies
        self.enemies.clear();
        self.enemies.push((350.0, 100.0, 50, true));
        self.enemies.push((700.0, 400.0, 50, false));
    }
}

impl Default for DemoGame {
    fn default() -> Self {
        Self::new()
    }
}

/// State for rendering the demo game
pub struct DemoRenderState {
    pub player_pos: Vec2,
    pub player_facing_right: bool,
    pub player_health_percent: f32,
    pub platforms: Vec<(f32, f32, f32, f32)>,
    pub coins: Vec<(f32, f32)>,
    pub enemies: Vec<(f32, f32, bool)>, // x, y, facing_right
    pub score: u32,
    pub game_over: bool,
    pub level_complete: bool,
}
