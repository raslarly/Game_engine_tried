# Getting Started with Phoenix Engine

This guide will walk you through setting up Phoenix Engine and creating your first 2D game.

## Prerequisites

Before you begin, make sure you have:

1. **Rust** (1.75 or later)
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Or on Windows, download from https://rustup.rs
   ```

2. **Python** (3.8-3.12, optional)
   - Download from [python.org](https://python.org)
   - Only needed if you want Python scripting support

## Building the Engine

1. Clone or navigate to the phoenix_engine directory:
   ```bash
   cd phoenix_engine
   ```

2. Build in release mode for best performance:
   ```bash
   cargo build --release
   ```

3. Run the engine:
   ```bash
   # Without Python support
   cargo run
   
   # With Python scripting
   cargo run --features python
   ```

## Your First Game

Let's create a simple game that displays a moving sprite.

### Step 1: Create a New Project

Create a new source file or modify the existing example:

```rust
// src/my_game.rs

use phoenix_engine::engine::{
    World, Transform, Renderer, Time, InputManager,
    ecs::{Name, Active},
    renderer::Sprite,
};
use glam::Vec2;

pub struct MyGame {
    world: World,
    time: Time,
    input: InputManager,
    player_id: Option<phoenix_engine::engine::ecs::EntityId>,
}

impl MyGame {
    pub fn new() -> Self {
        let mut input = InputManager::new();
        input.setup_default_bindings();
        
        Self {
            world: World::new(),
            time: Time::new(),
            input,
            player_id: None,
        }
    }
    
    pub fn setup(&mut self) {
        // Create a player entity
        let player = self.world.spawn_named("player", (
            Name::from("Player"),
            Transform::new(400.0, 300.0),
            Sprite::new("player_sprite"),
            Active::enabled(),
        ));
        
        self.player_id = Some(player);
    }
    
    pub fn update(&mut self, delta: f32) {
        // Get movement input
        let horizontal = self.input.get_axis("horizontal");
        let vertical = self.input.get_axis("vertical");
        
        // Move the player
        if let Some(id) = self.player_id {
            if let Some(mut transform) = self.world.get_mut::<Transform>(id) {
                let speed = 200.0;
                transform.position.x += horizontal * speed * delta;
                transform.position.y += vertical * speed * delta;
            }
        }
    }
}
```

### Step 2: Understanding the Game Loop

Phoenix Engine uses a standard game loop pattern:

```rust
fn game_loop() {
    let mut time = Time::new();
    
    loop {
        // 1. Update time
        time.update();
        let delta = time.delta();
        
        // 2. Process input
        // input.update(&ctx);
        
        // 3. Fixed update (physics)
        while time.should_fixed_update() {
            // physics_update(time.fixed_delta());
        }
        
        // 4. Game logic update
        // game.update(delta);
        
        // 5. Render
        // renderer.draw_all();
    }
}
```

### Step 3: Adding Sprites

Sprites are the visual representation of your game objects:

```rust
use phoenix_engine::engine::renderer::{Sprite, Animation, AnimationFrame};

// Create a simple sprite
let sprite = Sprite::new("hero")
    .with_color(1.0, 1.0, 1.0, 1.0);  // White tint (no tinting)

// Create an animated sprite
let walk_animation = Animation::new(
    vec![
        AnimationFrame { 
            texture_id: "hero_walk_1".to_string(), 
            source_rect: None, 
            duration: Some(0.1) 
        },
        AnimationFrame { 
            texture_id: "hero_walk_2".to_string(), 
            source_rect: None, 
            duration: Some(0.1) 
        },
        AnimationFrame { 
            texture_id: "hero_walk_3".to_string(), 
            source_rect: None, 
            duration: Some(0.1) 
        },
    ],
    0.1,  // Default frame time
);
```

### Step 4: Handling Input

Phoenix Engine uses an action-based input system:

```rust
use phoenix_engine::engine::input::{InputManager, InputAction, KeyBinding};

let mut input = InputManager::new();

// Setup default bindings (WASD, Space, etc.)
input.setup_default_bindings();

// Or create custom bindings
input.register_action(InputAction {
    name: "special_attack".to_string(),
    keys: vec![
        KeyBinding { 
            key: "Q".to_string(), 
            modifiers: Default::default() 
        },
    ],
    mouse_buttons: vec![],
});

// In your update loop
if input.is_action_pressed("jump") {
    // Player pressed jump this frame
}

if input.is_action_down("attack") {
    // Player is holding attack
}

// Get analog input
let h = input.get_axis("horizontal");  // -1.0 to 1.0
let v = input.get_axis("vertical");    // -1.0 to 1.0
```

### Step 5: Adding Physics

Add physics to make objects move and collide:

```rust
use phoenix_engine::engine::physics::{Rigidbody, Collider, PhysicsWorld, PhysicsConfig};

// Create a physics world
let physics = PhysicsWorld::new(PhysicsConfig {
    gravity: Vec2::new(0.0, -980.0),
    velocity_iterations: 8,
});

// Add physics components to an entity
let player = world.spawn((
    Transform::new(100.0, 300.0),
    Rigidbody::default(),
    Collider::box_collider(32.0, 48.0),
));

// In your fixed update
physics.update_body(&mut rigidbody, &mut transform.position, &mut transform.rotation, fixed_delta);

// Check collisions
if let Some(collision) = PhysicsWorld::check_collision(
    &collider_a, pos_a,
    &collider_b, pos_b,
) {
    // Handle collision
    println!("Collision! Normal: {:?}, Depth: {}", collision.normal, collision.depth);
}
```

## Next Steps

Now that you have the basics, explore these topics:

- [Core Concepts](./core-concepts.md) - Deep dive into engine architecture
- [Entity Component System](./ecs.md) - Advanced ECS patterns
- [Game Systems](./game-systems.md) - Player, combat, and inventory systems
- [Editor Guide](./editor.md) - Using the visual editor

## Troubleshooting

### Common Issues

**Build fails with missing dependencies:**
```bash
# On Windows, you may need Visual Studio Build Tools
# On Linux, install development packages:
sudo apt install pkg-config libfontconfig1-dev
```

**Python integration not working:**
```bash
# Ensure Python is in PATH and version is compatible (3.8-3.12)
python --version

# Build with Python feature
cargo build --features python
```

**Performance issues:**
```bash
# Always use release builds for testing performance
cargo run --release
```

---

[← Back to Index](./index.md) | [Core Concepts →](./core-concepts.md)
