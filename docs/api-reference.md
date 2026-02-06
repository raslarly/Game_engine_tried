# API Reference

Complete API reference for Phoenix Engine modules and types.

## Module Overview

| Module | Description |
|--------|-------------|
| `engine::ecs` | Entity Component System |
| `engine::transform` | Position, rotation, scale |
| `engine::renderer` | Sprites, animations, camera |
| `engine::physics` | Colliders, rigidbodies |
| `engine::input` | Keyboard, mouse, actions |
| `engine::audio` | Music and sound effects |
| `engine::resources` | Asset loading |
| `engine::time` | Delta time, timers |
| `game::player` | Player controller |
| `game::combat` | Combat mechanics |
| `game::inventory` | Items and equipment |
| `game::dialogue` | NPC conversations |

---

## engine::ecs

### World

The main container for all entities and components.

```rust
pub struct World {
    pub ecs: HecsWorld,
    // ...
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self;
    
    /// Spawn a new entity with components
    pub fn spawn<C: DynamicBundle>(&mut self, components: C) -> EntityId;
    
    /// Spawn a named entity
    pub fn spawn_named<C: DynamicBundle>(&mut self, name: &str, components: C) -> EntityId;
    
    /// Get entity by its ID
    pub fn get_entity(&self, id: EntityId) -> Option<Entity>;
    
    /// Get entity by name
    pub fn get_named(&self, name: &str) -> Option<EntityId>;
    
    /// Despawn an entity
    pub fn despawn(&mut self, id: EntityId) -> bool;
    
    /// Get a component from an entity
    pub fn get<T: Component>(&self, id: EntityId) -> Option<Ref<T>>;
    
    /// Get a mutable component from an entity
    pub fn get_mut<T: Component>(&self, id: EntityId) -> Option<RefMut<T>>;
    
    /// Add a component to an entity
    pub fn add_component<T: Component>(&mut self, id: EntityId, component: T) -> bool;
    
    /// Remove a component from an entity
    pub fn remove_component<T: Component>(&mut self, id: EntityId) -> Option<T>;
    
    /// Get the count of entities
    pub fn entity_count(&self) -> usize;
    
    /// Clear all entities
    pub fn clear(&mut self);
}
```

### EntityId

Unique identifier for entities.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self;
    pub fn from_uuid(uuid: Uuid) -> Self;
}
```

### Common Components

```rust
/// Entity name
pub struct Name(pub String);

/// Tag for grouping
pub struct Tag(pub String);

/// Active state
pub struct Active(pub bool);

/// Render layer
pub struct Layer(pub i32);
```

---

## engine::transform

### Transform

2D transform component.

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,      // Radians
    pub scale: Vec2,
    pub z_order: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32) -> Self;
    pub fn from_position(position: Vec2) -> Self;
    pub fn with_scale(self, scale: Vec2) -> Self;
    pub fn with_rotation(self, rotation: f32) -> Self;
    pub fn with_z_order(self, z_order: f32) -> Self;
    
    /// Get the transformation matrix
    pub fn matrix(&self) -> Mat3;
    
    /// Transform a local point to world space
    pub fn transform_point(&self, point: Vec2) -> Vec2;
    
    /// Get forward direction
    pub fn forward(&self) -> Vec2;
    
    /// Get right direction
    pub fn right(&self) -> Vec2;
    
    /// Translate by offset
    pub fn translate(&mut self, offset: Vec2);
    
    /// Rotate by angle (radians)
    pub fn rotate(&mut self, angle: f32);
    
    /// Look at target position
    pub fn look_at(&mut self, target: Vec2);
    
    /// Interpolate between transforms
    pub fn lerp(&self, other: &Transform, t: f32) -> Transform;
}
```

### Velocity

Velocity component.

```rust
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Velocity {
    pub linear: Vec2,
    pub angular: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self;
    pub fn from_linear(linear: Vec2) -> Self;
    pub fn with_angular(self, angular: f32) -> Self;
}
```

---

## engine::renderer

### Sprite

Sprite rendering component.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub texture_id: String,
    pub source_rect: Option<SpriteRect>,
    pub color: [f32; 4],    // RGBA tint
    pub flip_x: bool,
    pub flip_y: bool,
    pub pivot: [f32; 2],    // 0-1, default: [0.5, 0.5]
    pub visible: bool,
}

impl Sprite {
    pub fn new(texture_id: &str) -> Self;
    pub fn with_color(self, r: f32, g: f32, b: f32, a: f32) -> Self;
    pub fn with_source_rect(self, x: f32, y: f32, w: f32, h: f32) -> Self;
    pub fn flipped_x(self) -> Self;
    pub fn flipped_y(self) -> Self;
}
```

### Animation

Sprite animation component.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame: usize,
    pub frame_time: f32,
    pub elapsed: f32,
    pub looping: bool,
    pub playing: bool,
}

impl Animation {
    pub fn new(frames: Vec<AnimationFrame>, frame_time: f32) -> Self;
    pub fn update(&mut self, delta: f32);
    pub fn current_frame_data(&self) -> Option<&AnimationFrame>;
    pub fn play(&mut self);
    pub fn pause(&mut self);
    pub fn reset(&mut self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub texture_id: String,
    pub source_rect: Option<SpriteRect>,
    pub duration: Option<f32>,
}
```

### Camera2D

2D camera for view control.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera2D {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    pub viewport_size: Vec2,
}

impl Camera2D {
    pub fn new(width: f32, height: f32) -> Self;
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2;
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2;
    pub fn visible_bounds(&self) -> (Vec2, Vec2);
    pub fn follow(&mut self, target: Vec2, smoothness: f32, delta: f32);
}
```

### Renderer

Main renderer struct.

```rust
pub struct Renderer {
    pub camera: Camera2D,
    pub clear_color: Color32,
    pub debug_draw: bool,
}

impl Renderer {
    pub fn new() -> Self;
    pub fn load_texture(&mut self, ctx: &Context, id: &str, data: &[u8]) -> Result<(), String>;
    pub fn get_texture(&self, id: &str) -> Option<&TextureHandle>;
    pub fn draw_sprite(&self, painter: &Painter, sprite: &Sprite, position: Vec2, scale: Vec2, rotation: f32);
    pub fn draw_debug_rect(&self, painter: &Painter, min: Vec2, max: Vec2, color: Color32);
    pub fn draw_debug_circle(&self, painter: &Painter, center: Vec2, radius: f32, color: Color32);
}
```

---

## engine::physics

### AABB

Axis-aligned bounding box.

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self;
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self;
    pub fn center(&self) -> Vec2;
    pub fn size(&self) -> Vec2;
    pub fn contains_point(&self, point: Vec2) -> bool;
    pub fn intersects(&self, other: &AABB) -> bool;
    pub fn translate(&self, offset: Vec2) -> AABB;
}
```

### Collider

Collision shape component.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Collider {
    Box { width: f32, height: f32, offset: Vec2 },
    Circle { radius: f32, offset: Vec2 },
}

impl Collider {
    pub fn box_collider(width: f32, height: f32) -> Self;
    pub fn circle_collider(radius: f32) -> Self;
    pub fn with_offset(self, offset: Vec2) -> Self;
    pub fn get_aabb(&self, position: Vec2) -> AABB;
}
```

### Rigidbody

Physics body component.

```rust
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

impl Rigidbody {
    pub fn kinematic() -> Self;
    pub fn apply_force(&mut self, force: Vec2);
    pub fn apply_impulse(&mut self, impulse: Vec2);
}
```

### PhysicsWorld

Physics simulation.

```rust
pub struct PhysicsWorld {
    pub config: PhysicsConfig,
}

impl PhysicsWorld {
    pub fn new(config: PhysicsConfig) -> Self;
    pub fn update_body(&self, rb: &mut Rigidbody, position: &mut Vec2, rotation: &mut f32, dt: f32);
    pub fn check_collision(a: &Collider, pos_a: Vec2, b: &Collider, pos_b: Vec2) -> Option<CollisionInfo>;
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    pub normal: Vec2,
    pub depth: f32,
    pub point: Vec2,
}
```

---

## engine::input

### InputManager

Input handling system.

```rust
pub struct InputManager {
    // ...
}

impl InputManager {
    pub fn new() -> Self;
    pub fn update(&mut self, ctx: &Context);
    pub fn setup_default_bindings(&mut self);
    
    // Key queries
    pub fn is_key_down(&self, key: Key) -> bool;
    pub fn is_key_pressed(&self, key: Key) -> bool;
    pub fn is_key_released(&self, key: Key) -> bool;
    
    // Mouse queries
    pub fn is_mouse_button_down(&self, button: PointerButton) -> bool;
    pub fn is_mouse_button_pressed(&self, button: PointerButton) -> bool;
    pub fn mouse_position(&self) -> Vec2;
    pub fn mouse_delta(&self) -> Vec2;
    pub fn scroll_delta(&self) -> Vec2;
    
    // Modifiers
    pub fn is_ctrl_down(&self) -> bool;
    pub fn is_shift_down(&self) -> bool;
    pub fn is_alt_down(&self) -> bool;
    
    // Actions
    pub fn register_action(&mut self, action: InputAction);
    pub fn register_axis(&mut self, axis: InputAxis);
    pub fn is_action_pressed(&self, name: &str) -> bool;
    pub fn is_action_down(&self, name: &str) -> bool;
    pub fn get_axis(&self, name: &str) -> f32;
}
```

---

## engine::audio

### AudioManager

Audio playback system.

```rust
pub struct AudioManager {
    // ...
}

impl AudioManager {
    pub fn new() -> Result<Self, String>;
    pub fn load_clip(&mut self, id: &str, data: Vec<u8>);
    pub fn play_sfx(&mut self, id: &str) -> Result<(), String>;
    pub fn play_sfx_with_volume(&mut self, id: &str, volume: f32) -> Result<(), String>;
    pub fn play_music(&mut self, id: &str) -> Result<(), String>;
    pub fn stop_music(&mut self);
    pub fn pause_music(&self);
    pub fn resume_music(&self);
    pub fn set_master_volume(&mut self, volume: f32);
    pub fn set_music_volume(&mut self, volume: f32);
    pub fn set_sfx_volume(&mut self, volume: f32);
    pub fn is_music_playing(&self) -> bool;
}
```

---

## engine::time

### Time

Time management.

```rust
pub struct Time {
    // ...
}

impl Time {
    pub fn new() -> Self;
    pub fn update(&mut self);
    pub fn should_fixed_update(&mut self) -> bool;
    pub fn delta(&self) -> f32;
    pub fn unscaled_delta(&self) -> f32;
    pub fn time_scale(&self) -> f32;
    pub fn set_time_scale(&mut self, scale: f32);
    pub fn fixed_delta(&self) -> f32;
    pub fn set_fixed_timestep(&mut self, timestep: f32);
    pub fn elapsed(&self) -> f32;
    pub fn frame_count(&self) -> u64;
    pub fn fps(&self) -> f32;
    pub fn fixed_alpha(&self) -> f32;
}
```

### Timer

Timer utility.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    // ...
}

impl Timer {
    pub fn new(duration: f32, repeating: bool) -> Self;
    pub fn once(duration: f32) -> Self;
    pub fn repeating(duration: f32) -> Self;
    pub fn update(&mut self, delta: f32) -> bool;
    pub fn reset(&mut self);
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn progress(&self) -> f32;
    pub fn remaining(&self) -> f32;
    pub fn is_finished(&self) -> bool;
}
```

---

## engine::resources

### ResourceManager

Asset loading and caching.

```rust
pub struct ResourceManager {
    // ...
}

impl ResourceManager {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self;
    pub fn load_file(&mut self, id: &str, path: &str, resource_type: ResourceType) -> Result<(), String>;
    pub fn get_data(&self, id: &str) -> Option<&Vec<u8>>;
    pub fn is_loaded(&self, id: &str) -> bool;
    pub fn unload(&mut self, id: &str);
    pub fn load_directory(&mut self, dir: &str) -> Result<Vec<String>, String>;
    pub fn set_base_path<P: AsRef<Path>>(&mut self, path: P);
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Texture,
    Audio,
    Font,
    Data,
    Script,
}
```

---

For more detailed examples, see the [Examples](../examples/) directory.
