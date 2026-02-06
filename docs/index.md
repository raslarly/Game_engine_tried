# Phoenix Engine Documentation 🔥

Welcome to the Phoenix Engine documentation! This guide will help you build your own indie 2D games using Phoenix Engine's powerful features.

## Table of Contents

1. [Getting Started](./getting-started.md) - Installation and first project
2. [Core Concepts](./core-concepts.md) - Understanding the engine architecture
3. [Entity Component System](./ecs.md) - Working with entities and components
4. [Rendering](./rendering.md) - Sprites, animations, and camera
5. [Physics](./physics.md) - Colliders, rigidbodies, and collision detection
6. [Input System](./input.md) - Keyboard, mouse, and action bindings
7. [Audio](./audio.md) - Music and sound effects
8. [Game Systems](./game-systems.md) - Player, combat, inventory, and dialogue
9. [Python Scripting](./python-scripting.md) - Configuration and automation
10. [Editor Guide](./editor.md) - Using the visual editor
11. [API Reference](./api-reference.md) - Complete API documentation

## Quick Start

```rust
use phoenix_engine::engine::{World, Transform, Renderer, Time, InputManager};
use phoenix_engine::engine::ecs::Name;

fn main() {
    // Create the game world
    let mut world = World::new();
    
    // Spawn a player entity
    let player_id = world.spawn_named("player", (
        Name::from("Hero"),
        Transform::new(100.0, 100.0),
    ));
    
    // Your game loop here...
}
```

## Features at a Glance

| Feature | Description |
|---------|-------------|
| **ECS Architecture** | Efficient entity management using the `hecs` library |
| **2D Rendering** | Sprite rendering, animations, sprite sheets, and camera system |
| **Physics** | AABB and circle colliders, rigidbodies, collision detection |
| **Input System** | Action-based input with keyboard and mouse support |
| **Audio** | Music playback, sound effects, and volume control |
| **Visual Editor** | EGUI-based editor for scene creation |
| **Python Scripting** | Optional Python integration for configuration |

## System Requirements

- **Rust**: 1.75 or later
- **Python**: 3.8-3.12 (optional, for Python scripting)
- **OS**: Windows, macOS, or Linux

## Project Structure

```
phoenix_engine/
├── src/
│   ├── main.rs           # Application entry point
│   ├── engine/           # Core engine systems
│   ├── editor/           # Visual editor
│   ├── scripting/        # Python integration
│   └── game/             # Game-specific systems
├── examples/             # Example games
├── assets/               # Game assets
├── scripts/              # Python configuration scripts
├── config/               # Configuration files
└── docs/                 # Documentation
```

## License

Phoenix Engine is released under the MIT License.

---

Continue to [Getting Started →](./getting-started.md)
