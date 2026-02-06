# Phoenix Engine 🔥

A modern 2D action-adventure game engine built with **Rust**, **EGUI**, and **Python** scripting support.

## Features

### Core Engine
- **Entity Component System (ECS)** - Efficient game object management using `hecs`
- **Transform System** - Position, rotation, scale, and hierarchies
- **2D Rendering** - Sprite rendering with animations, layers, and camera
- **Physics** - Simple 2D physics with collision detection
- **Audio** - Music and sound effects using `rodio`
- **Input** - Keyboard, mouse, and action-based input system

### Editor
- **Visual Scene Editor** - Intuitive EGUI-based interface
- **Hierarchy Panel** - View and organize game objects
- **Inspector Panel** - Edit component properties
- **Asset Browser** - Manage sprites, audio, and scripts
- **Console** - Debug logging and script output

### Python Scripting
- **Configuration via Python** - Edit game balance with Python scripts
- **PyO3 Integration** - Full Python runtime embedded in the engine
- **Hot Reloading** - Changes apply without restarting

### Game Systems
- **Player Controller** - Platformer mechanics with jump, dash, attack
- **Combat System** - Health, damage, hitboxes, enemy AI
- **Inventory** - Items, equipment, and stacking
- **Dialogue** - NPC conversations with branching options

## Getting Started

### Prerequisites

- **Rust** (1.75 or later): https://rustup.rs/
- **Python** (3.8 or later): https://python.org/

### Building

```bash
cd phoenix_engine
cargo build --release
```

### Running

Run the engine with default settings (Python disabled):
```bash
cargo run
```

To enable Python scripting support (requires compatible Python 3.8-3.12 installed):
```bash
cargo run --features python
```

## Project Structure

```
phoenix_engine/
├── src/
│   ├── main.rs           # Application entry point
│   ├── engine/           # Core engine systems
│   │   ├── ecs.rs        # Entity Component System
│   │   ├── transform.rs  # Transform component
│   │   ├── renderer.rs   # 2D rendering
│   │   ├── audio.rs      # Audio playback
│   │   ├── input.rs      # Input handling
│   │   ├── physics.rs    # Physics simulation
│   │   ├── resources.rs  # Asset loading
│   │   └── time.rs       # Time management
│   ├── editor/           # Visual editor
│   │   ├── app.rs        # Main editor application
│   │   ├── panels.rs     # UI panels
│   │   ├── project.rs    # Project management
│   │   └── scene.rs      # Scene serialization
│   ├── scripting/        # Python integration
│   │   ├── runtime.rs    # Python interpreter
│   │   ├── config.rs     # Configuration loading
│   │   └── api.rs        # Python API bindings
│   └── game/             # Game-specific systems
│       ├── player.rs     # Player controller
│       ├── combat.rs     # Combat mechanics
│       ├── inventory.rs  # Item management
│       └── dialogue.rs   # NPC dialogue
├── examples/             # Example games
│   └── platformer_demo/  # Complete platformer demo
│       ├── mod.rs
│       ├── demo_game.rs  # Main game logic
│       ├── demo_player.rs # Player controller
│       ├── demo_enemy.rs # Enemy AI
│       └── demo_level.rs # Level builder
├── docs/                 # Documentation
│   ├── index.md          # Documentation home
│   ├── getting-started.md # Quick start guide
│   ├── core-concepts.md  # Architecture overview
│   ├── game-systems.md   # Player, combat, inventory
│   └── api-reference.md  # Complete API docs
├── assets/               # Game assets
│   ├── sprites/          # Sprite images
│   ├── audio/            # Sound files
│   └── fonts/            # Font files
├── scripts/              # Python scripts
│   ├── game_config.py    # Game configuration
│   ├── input_config.py   # Input bindings
│   └── level_editor.py   # Level creation
└── config/               # Configuration files
    ├── project.toml      # Project settings
    └── game.json         # Game configuration
```

## Python Scripting

### Configuration Example

Create a `scripts/game_config.py`:

```python
def get_config():
    return {
        "player": {
            "max_health": 100,
            "move_speed": 200.0,
            "jump_force": 500.0,
        },
        "enemies": {
            "slime": {
                "name": "Slime",
                "max_health": 30,
                "attack_damage": 5,
            }
        }
    }
```

### Level Creation

```python
from level_editor import Level, Platform, EnemySpawner

level = Level("Forest")
level.set_spawn(0, 50)
level.add_entity(Platform("Ground", 0, -25, 1000, 50))
level.add_entity(EnemySpawner("Slime1", "slime", 200, 50))
```

## Controls

| Action | Default Binding |
|--------|-----------------|
| Move | WASD / Arrow Keys |
| Jump | Space |
| Attack | J / Left Click |
| Dash | K / Shift+Space |
| Interact | E |
| Pause | Escape |
| Inventory | I / Tab |

## License

MIT License - See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

---

Built with ❤️ using Rust, EGUI, and Python
