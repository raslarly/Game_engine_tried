//! Demo Level Builder
//! 
//! Example level creation for Phoenix Engine games.

use glam::Vec2;

/// A platform in the level
#[derive(Debug, Clone)]
pub struct DemoPlatform {
    pub position: Vec2,
    pub size: Vec2,
    pub is_one_way: bool,
    pub is_moving: bool,
    pub move_path: Option<(Vec2, Vec2)>,
}

impl DemoPlatform {
    /// Create a static platform
    pub fn static_platform(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
            is_one_way: false,
            is_moving: false,
            move_path: None,
        }
    }
    
    /// Create a one-way platform (can jump through from below)
    pub fn one_way(x: f32, y: f32, width: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, 10.0),
            is_one_way: true,
            is_moving: false,
            move_path: None,
        }
    }
    
    /// Create a moving platform
    pub fn moving(x: f32, y: f32, width: f32, end_x: f32, end_y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, 20.0),
            is_one_way: false,
            is_moving: true,
            move_path: Some((Vec2::new(x, y), Vec2::new(end_x, end_y))),
        }
    }
    
    /// Get bounds for collision
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        (self.position, self.position + self.size)
    }
}

/// A collectible item in the level
#[derive(Debug, Clone)]
pub struct DemoCollectible {
    pub position: Vec2,
    pub collectible_type: CollectibleType,
    pub collected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectibleType {
    Coin,       // +100 points
    Gem,        // +500 points
    HealthPack, // Heal 50 HP
    PowerUp,    // Special ability
}

impl CollectibleType {
    pub fn point_value(&self) -> u32 {
        match self {
            CollectibleType::Coin => 100,
            CollectibleType::Gem => 500,
            CollectibleType::HealthPack => 50,  // Not really points
            CollectibleType::PowerUp => 1000,
        }
    }
}

/// A complete demo level
pub struct DemoLevel {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub spawn_point: Vec2,
    pub exit_point: Vec2,
    pub platforms: Vec<DemoPlatform>,
    pub collectibles: Vec<DemoCollectible>,
    pub enemy_spawns: Vec<(Vec2, super::demo_enemy::DemoEnemyType)>,
    pub background_color: [u8; 3],
}

impl DemoLevel {
    /// Create a new empty level
    pub fn new(name: &str, width: f32, height: f32) -> Self {
        Self {
            name: name.to_string(),
            width,
            height,
            spawn_point: Vec2::new(100.0, 200.0),
            exit_point: Vec2::new(width - 100.0, 200.0),
            platforms: Vec::new(),
            collectibles: Vec::new(),
            enemy_spawns: Vec::new(),
            background_color: [30, 40, 60],
        }
    }
    
    /// Add a platform
    pub fn add_platform(&mut self, platform: DemoPlatform) {
        self.platforms.push(platform);
    }
    
    /// Add ground
    pub fn add_ground(&mut self) {
        self.platforms.push(DemoPlatform::static_platform(
            0.0, 0.0, self.width, 50.0
        ));
    }
    
    /// Add a coin at position
    pub fn add_coin(&mut self, x: f32, y: f32) {
        self.collectibles.push(DemoCollectible {
            position: Vec2::new(x, y),
            collectible_type: CollectibleType::Coin,
            collected: false,
        });
    }
    
    /// Add an enemy spawn
    pub fn add_enemy(&mut self, x: f32, y: f32, enemy_type: super::demo_enemy::DemoEnemyType) {
        self.enemy_spawns.push((Vec2::new(x, y), enemy_type));
    }
    
    /// Set spawn point
    pub fn set_spawn(&mut self, x: f32, y: f32) {
        self.spawn_point = Vec2::new(x, y);
    }
    
    /// Set exit point
    pub fn set_exit(&mut self, x: f32, y: f32) {
        self.exit_point = Vec2::new(x, y);
    }
}

/// Create the first demo level
pub fn create_level_1() -> DemoLevel {
    let mut level = DemoLevel::new("Forest Beginnings", 2000.0, 800.0);
    
    // Ground
    level.add_ground();
    
    // Platforms
    level.add_platform(DemoPlatform::static_platform(200.0, 150.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::static_platform(450.0, 250.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::one_way(700.0, 350.0, 120.0));
    level.add_platform(DemoPlatform::static_platform(900.0, 250.0, 200.0, 20.0));
    level.add_platform(DemoPlatform::moving(1200.0, 200.0, 100.0, 1200.0, 400.0));
    level.add_platform(DemoPlatform::static_platform(1400.0, 450.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::static_platform(1650.0, 350.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::static_platform(1850.0, 250.0, 150.0, 20.0));
    
    // Coins
    level.add_coin(275.0, 200.0);
    level.add_coin(525.0, 300.0);
    level.add_coin(760.0, 400.0);
    level.add_coin(1000.0, 300.0);
    level.add_coin(1250.0, 300.0);
    level.add_coin(1475.0, 500.0);
    level.add_coin(1725.0, 400.0);
    level.add_coin(1925.0, 300.0);
    
    // Enemies
    use super::demo_enemy::DemoEnemyType;
    level.add_enemy(500.0, 100.0, DemoEnemyType::Slime);
    level.add_enemy(950.0, 300.0, DemoEnemyType::Skeleton);
    level.add_enemy(1500.0, 500.0, DemoEnemyType::Slime);
    level.add_enemy(1700.0, 400.0, DemoEnemyType::Bat);
    
    // Spawn and exit
    level.set_spawn(100.0, 150.0);
    level.set_exit(1900.0, 300.0);
    
    level.background_color = [20, 50, 30]; // Forest green tint
    
    level
}

/// Create the second demo level
pub fn create_level_2() -> DemoLevel {
    let mut level = DemoLevel::new("Cave Depths", 2500.0, 1000.0);
    
    // More challenging level with more enemies and tricky platforming
    level.add_ground();
    
    // Complex platform layout
    level.add_platform(DemoPlatform::static_platform(150.0, 150.0, 100.0, 20.0));
    level.add_platform(DemoPlatform::static_platform(350.0, 250.0, 80.0, 20.0));
    level.add_platform(DemoPlatform::one_way(500.0, 350.0, 60.0));
    level.add_platform(DemoPlatform::moving(650.0, 200.0, 80.0, 650.0, 500.0));
    level.add_platform(DemoPlatform::static_platform(850.0, 400.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::one_way(1100.0, 300.0, 100.0));
    level.add_platform(DemoPlatform::static_platform(1300.0, 500.0, 120.0, 20.0));
    level.add_platform(DemoPlatform::moving(1500.0, 350.0, 100.0, 1700.0, 350.0));
    level.add_platform(DemoPlatform::static_platform(1900.0, 450.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::static_platform(2150.0, 350.0, 150.0, 20.0));
    level.add_platform(DemoPlatform::static_platform(2350.0, 250.0, 100.0, 20.0));
    
    // More enemies
    use super::demo_enemy::DemoEnemyType;
    level.add_enemy(400.0, 100.0, DemoEnemyType::Slime);
    level.add_enemy(600.0, 100.0, DemoEnemyType::Skeleton);
    level.add_enemy(900.0, 450.0, DemoEnemyType::Bat);
    level.add_enemy(1150.0, 350.0, DemoEnemyType::Slime);
    level.add_enemy(1350.0, 550.0, DemoEnemyType::Skeleton);
    level.add_enemy(1800.0, 400.0, DemoEnemyType::Bat);
    level.add_enemy(2000.0, 500.0, DemoEnemyType::Skeleton);
    level.add_enemy(2200.0, 400.0, DemoEnemyType::Bat);
    
    level.set_spawn(80.0, 150.0);
    level.set_exit(2400.0, 300.0);
    
    level.background_color = [25, 25, 35]; // Dark cave
    
    level
}
