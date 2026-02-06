# Core Concepts

Understanding Phoenix Engine's architecture will help you build better games. This guide covers the fundamental concepts that power the engine.

## Engine Architecture

Phoenix Engine follows a modular architecture with these core systems:

```
┌─────────────────────────────────────────────────────────────┐
│                      Phoenix Engine                          │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   ECS    │  │ Renderer │  │  Physics │  │  Audio   │   │
│  │  World   │  │          │  │  World   │  │ Manager  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Input   │  │   Time   │  │ Resource │  │  Python  │   │
│  │ Manager  │  │          │  │ Manager  │  │ Runtime  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
├─────────────────────────────────────────────────────────────┤
│                     Game Systems                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Player  │  │  Combat  │  │ Inventory│  │ Dialogue │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## The Game Loop

Every frame, your game goes through these phases:

### 1. Time Update
```rust
time.update();
let delta = time.delta();  // Time since last frame
```

### 2. Input Processing
```rust
input.update(&egui_ctx);

if input.is_action_pressed("jump") {
    // Handle jump input
}
```

### 3. Fixed Update (Physics)
Fixed updates run at a consistent rate (default: 60 Hz) regardless of frame rate:

```rust
while time.should_fixed_update() {
    let fixed_dt = time.fixed_delta();
    
    // Update physics
    physics.update_body(&mut rb, &mut pos, &mut rot, fixed_dt);
    
    // Check collisions
    // Apply forces
}
```

### 4. Variable Update (Game Logic)
```rust
// Update animations
animation.update(delta);

// Update AI
enemy.update(delta, player_pos);

// Update timers
cooldown_timer.update(delta);
```

### 5. Render
```rust
// Clear screen
// Draw sprites (sorted by z-order)
// Draw UI
// Present frame
```

## Entity Component System (ECS)

Phoenix Engine uses an ECS architecture for game object management.

### Entities
Entities are unique identifiers that represent game objects:

```rust
let player_id = world.spawn((
    Name::from("Player"),
    Transform::new(100.0, 100.0),
    Sprite::new("player"),
));
```

### Components
Components are data containers attached to entities:

```rust
// Built-in components
Transform { position, rotation, scale, z_order }
Sprite { texture_id, color, flip_x, flip_y, visible }
Rigidbody { velocity, mass, gravity_scale, is_kinematic }
Collider { Box or Circle with offset }

// Custom components
#[derive(Debug, Clone)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Debug, Clone)]
struct Enemy {
    patrol_points: Vec<Vec2>,
    aggro_range: f32,
}
```

### Systems
Systems operate on entities that have specific component combinations:

```rust
// Pseudo-code for a movement system
fn movement_system(world: &mut World, delta: f32) {
    for (entity, (transform, velocity)) in world.ecs.query::<(&mut Transform, &Velocity)>() {
        transform.position += velocity.linear * delta;
        transform.rotation += velocity.angular * delta;
    }
}
```

## Coordinate System

Phoenix Engine uses a standard 2D coordinate system:

```
        Y ↑
          │
          │    (100, 100)
          │       ●
          │
 ─────────┼─────────→ X
          │
          │
```

- **Origin (0, 0)**: Center of the world (or as you define)
- **X-axis**: Positive = Right, Negative = Left
- **Y-axis**: Positive = Up, Negative = Down
- **Rotation**: Counter-clockwise, in radians

## Time Management

Understanding time is crucial for smooth gameplay:

```rust
let time = Time::new();

// In game loop
time.update();

// Delta time (affected by time scale)
let delta = time.delta();

// Unscaled delta (for UI, pause menus)
let unscaled = time.unscaled_delta();

// Slow motion effects
time.set_time_scale(0.5);  // Half speed
time.set_time_scale(1.0);  // Normal speed

// Fixed timestep (for physics)
let fixed_dt = time.fixed_delta();  // Default: 1/60 second

// Total elapsed time
let elapsed = time.elapsed();

// Current FPS
let fps = time.fps();
```

## Resource Management

Assets are loaded and cached through the ResourceManager:

```rust
let mut resources = ResourceManager::new("./assets");

// Load a texture
resources.load_file("player", "sprites/player.png", ResourceType::Texture)?;

// Load audio
resources.load_file("jump_sound", "audio/jump.wav", ResourceType::Audio)?;

// Load all files from a directory
let loaded = resources.load_directory("sprites")?;
println!("Loaded: {:?}", loaded);

// Access loaded data
if let Some(data) = resources.get_data("player") {
    // Use the raw bytes
}
```

## Transform Hierarchy

Transforms can be hierarchical for complex objects:

```rust
// Create a transform
let mut transform = Transform::new(100.0, 50.0)
    .with_scale(Vec2::new(2.0, 2.0))
    .with_rotation(std::f32::consts::PI / 4.0);  // 45 degrees

// Transform a local point to world space
let local_point = Vec2::new(10.0, 0.0);
let world_point = transform.transform_point(local_point);

// Get direction vectors
let forward = transform.forward();  // Direction entity is facing
let right = transform.right();      // Perpendicular direction

// Look at a target
transform.look_at(target_position);

// Smooth interpolation between transforms
let interpolated = transform_a.lerp(&transform_b, 0.5);
```

## Timers and Cooldowns

Use the Timer utility for delays and cooldowns:

```rust
use phoenix_engine::engine::time::Timer;

// One-shot timer (runs once)
let mut explosion_delay = Timer::once(2.0);  // 2 seconds

// Repeating timer
let mut spawn_timer = Timer::repeating(0.5);  // Every 0.5 seconds

// In update loop
if explosion_delay.update(delta) {
    // Timer finished! Trigger explosion
    spawn_explosion();
}

if spawn_timer.update(delta) {
    // Timer triggered! Spawn enemy
    spawn_enemy();
}

// Check progress
let progress = timer.progress();  // 0.0 to 1.0

// Control timer
timer.pause();
timer.resume();
timer.reset();
```

## Best Practices

### 1. Use Components Wisely
- Keep components small and focused
- Avoid putting logic in components
- Use components for data, systems for logic

### 2. Optimize Fixed Updates
- Put physics and collision code in fixed update
- Use variable update for rendering and input

### 3. Cache Entity Lookups
```rust
// Instead of looking up every frame
let player_id = world.get_named("player");  // Cache this

// Then use the cached ID
if let Some(transform) = world.get::<Transform>(player_id.unwrap()) {
    // Use transform
}
```

### 4. Use Object Pooling
For frequently created/destroyed objects (bullets, particles):
```rust
struct BulletPool {
    bullets: Vec<Option<Bullet>>,
}

impl BulletPool {
    fn get(&mut self) -> &mut Bullet {
        // Find inactive bullet or create new
    }
    
    fn release(&mut self, index: usize) {
        self.bullets[index] = None;
    }
}
```

---

[← Getting Started](./getting-started.md) | [Entity Component System →](./ecs.md)
