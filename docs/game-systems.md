# Game Systems

Phoenix Engine includes pre-built systems for common 2D action-adventure game mechanics. This guide covers the Player, Combat, Inventory, and Dialogue systems.

## Player System

The player system provides a complete platformer controller with movement, jumping, dashing, and attacking.

### Player Component

```rust
use phoenix_engine::game::player::{Player, PlayerState, PlayerController};

// Create a player with default stats
let mut player = Player::new();

// Or customize the player
player.max_health = 150;
player.move_speed = 250.0;
player.jump_force = 550.0;
player.attack_damage = 30;
```

### Player States

The player can be in these states:

| State | Description |
|-------|-------------|
| `Idle` | Standing still |
| `Walking` | Moving at normal speed |
| `Running` | Moving at run speed |
| `Jumping` | Moving upward from a jump |
| `Falling` | Moving downward |
| `Attacking` | Executing an attack |
| `Dashing` | Performing a dash |
| `Hurt` | Taking damage (invincible) |
| `Dead` | No health remaining |

### Movement and Jumping

```rust
use phoenix_engine::game::player::PlayerController;

// Calculate movement velocity
let movement = PlayerController::calculate_movement(
    &player,
    horizontal_input,  // -1.0 to 1.0
    is_running,        // Shift held
);

// Handle jumping
if input.is_action_pressed("jump") {
    player.buffer_jump();  // Buffer the input
}

// Try to consume the jump buffer
if player.consume_jump_buffer() {
    if let Some(jump_force) = PlayerController::calculate_jump(&mut player, is_grounded) {
        velocity.y = jump_force;
    }
}
```

### Dashing

```rust
// Try to dash
let dash_direction = Vec2::new(horizontal_input, vertical_input);
if input.is_action_pressed("dash") {
    if let Some(dash_velocity) = player.start_dash(dash_direction) {
        velocity = dash_velocity;
    }
}

// Check if still dashing
if player.state == PlayerState::Dashing && player.dash_duration.is_finished() {
    player.state = PlayerState::Falling;
}
```

### Combat

```rust
// Attack
if input.is_action_pressed("attack") {
    player.start_attack();
}

// Take damage
if player.take_damage(enemy_damage) {
    // Damage was applied
    play_sound("hurt");
    
    if !player.is_alive() {
        // Player died!
        game_over();
    }
}

// Heal
player.heal(25);

// Check invincibility (for visual effects)
if player.is_invincible() {
    // Flash the player sprite
}
```

### Coyote Time and Jump Buffering

Phoenix Engine includes platformer polish features:

- **Coyote Time**: Brief window after leaving a platform where jump is still valid
- **Jump Buffering**: If you press jump slightly before landing, it triggers when you land

```rust
// These are handled automatically when you use:
player.buffer_jump();
player.consume_jump_buffer();
player.can_jump();  // Accounts for coyote time
```

## Combat System

The combat system handles health, damage, and enemy AI.

### Health Component

```rust
use phoenix_engine::game::combat::Health;

let mut health = Health::new(100);  // 100 max HP

// In update loop
health.update(delta);

// Take damage (respects invincibility)
if health.take_damage(25) {
    // Damage was applied
    spawn_damage_number(25);
}

// Heal
health.heal(10);

// Check status
if !health.is_alive() {
    destroy_entity();
}

let percent = health.percent();  // For health bars
```

### Damage Types

```rust
use phoenix_engine::game::combat::{DamageDealer, DamageType};

// Player attack
let sword_hit = DamageDealer::player_attack(25);

// Enemy attack
let enemy_hit = DamageDealer::enemy_attack(10);

// Projectile
let arrow = DamageDealer::projectile(15, false);  // false = hits enemies

// Custom damage
let fire_spell = DamageDealer {
    damage: 40,
    damage_type: DamageType::Fire,
    knockback_force: 300.0,
    hits_player: false,
    hits_enemies: true,
    one_shot: true,
    has_hit: false,
};
```

### Hitboxes

```rust
use phoenix_engine::game::combat::Hitbox;

// Create a hitbox for an attack
let attack_hitbox = Hitbox::new(60.0, 40.0)
    .with_offset(Vec2::new(30.0, 0.0))  // Offset from entity center
    .with_lifetime(0.2);  // Disappears after 0.2 seconds

// Update hitbox
attack_hitbox.update(delta);

// Check if still active
if attack_hitbox.active {
    // Check collisions with enemies
}
```

### Enemy AI

```rust
use phoenix_engine::game::combat::{Enemy, EnemyState};

let mut enemy = Enemy::new("slime");

// Configure stats
enemy.max_health = 50;
enemy.move_speed = 80.0;
enemy.attack_damage = 15;
enemy.detection_range = 200.0;
enemy.attack_range = 40.0;

// Set patrol path
enemy.patrol_points = vec![
    Vec2::new(100.0, 50.0),
    Vec2::new(300.0, 50.0),
];

// In update loop
enemy.update(delta);

// Check if enemy can see player
if enemy.can_see_player(enemy_pos, player_pos) {
    enemy.state = EnemyState::Chase;
}

// Check if in attack range
if enemy.in_attack_range(enemy_pos, player_pos) && enemy.can_attack() {
    enemy.state = EnemyState::Attack;
}

// Enemy takes damage
if enemy.take_damage(25) {
    spawn_damage_number(25);
}
```

### Loot Drops

```rust
use phoenix_engine::game::combat::LootDrop;

// When enemy dies, create loot
let loot = LootDrop::new("gold_coin", 5);

// The loot has a random velocity for scatter effect
spawn_collectible(enemy_pos, loot);
```

## Inventory System

A flexible inventory system for items, equipment, and stacking.

### Basic Usage

```rust
use phoenix_engine::game::inventory::{Inventory, Item, ItemType, ItemRarity};

// Create an inventory with 20 slots
let mut inventory = Inventory::new(20);

// Add items
if inventory.add_item(Item::new("health_potion", "Health Potion", ItemType::Consumable)) {
    // Item added successfully
}

// Check quantities
let potion_count = inventory.count_item("health_potion");

// Use an item
if let Some(item) = inventory.remove_item("health_potion") {
    player.heal(50);
}
```

### Item Types

```rust
// Different item types
enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Material,
    Key,
    Quest,
}

// Item rarities
enum ItemRarity {
    Common,     // White
    Uncommon,   // Green
    Rare,       // Blue
    Epic,       // Purple
    Legendary,  // Orange
}
```

### Equipment System

```rust
use phoenix_engine::game::inventory::{Equipment, EquipmentSlot};

let mut equipment = Equipment::new();

// Equip items
let sword = Item::new("iron_sword", "Iron Sword", ItemType::Weapon);
equipment.equip(EquipmentSlot::Weapon, sword);

// Get equipped item
if let Some(weapon) = equipment.get(EquipmentSlot::Weapon) {
    let damage = weapon.stats.attack;
}

// Unequip
let old_weapon = equipment.unequip(EquipmentSlot::Weapon);
```

### Item Stacking

```rust
// Stackable items (like potions, arrows)
let mut potion = Item::new("health_potion", "Health Potion", ItemType::Consumable);
potion.max_stack = 99;
potion.current_stack = 1;

// Adding more of the same item stacks them
inventory.add_item(potion.clone());  // Stack becomes 2
inventory.add_item(potion.clone());  // Stack becomes 3
```

## Dialogue System

Create branching conversations with NPCs.

### Basic Dialogue

```rust
use phoenix_engine::game::dialogue::{DialogueManager, DialogueNode, DialogueChoice};

let mut dialogue = DialogueManager::new();

// Create a dialogue tree
dialogue.add_node("start", DialogueNode {
    speaker: "Old Man".to_string(),
    text: "Hello, young adventurer! Are you ready for a quest?".to_string(),
    choices: vec![
        DialogueChoice {
            text: "Yes, I'm ready!".to_string(),
            next_node: Some("quest_accept".to_string()),
            condition: None,
        },
        DialogueChoice {
            text: "Not right now.".to_string(),
            next_node: Some("quest_decline".to_string()),
            condition: None,
        },
    ],
    on_enter: None,
});

// Start dialogue
dialogue.start("start");

// Get current node
if let Some(node) = dialogue.current_node() {
    display_dialogue(node.speaker, node.text);
    
    for (i, choice) in node.choices.iter().enumerate() {
        display_choice(i, choice.text);
    }
}

// Select a choice
dialogue.select_choice(0);  // Choose first option
```

### Conditional Choices

```rust
// Choice that only appears if player has an item
DialogueChoice {
    text: "I have the ancient key!".to_string(),
    next_node: Some("open_door".to_string()),
    condition: Some("has_ancient_key".to_string()),
}

// Check conditions
fn check_condition(condition: &str) -> bool {
    match condition {
        "has_ancient_key" => inventory.has_item("ancient_key"),
        "quest_complete" => quest_log.is_complete("main_quest"),
        _ => true,
    }
}
```

### Dialogue Events

```rust
// Trigger events when entering a node
DialogueNode {
    speaker: "Merchant".to_string(),
    text: "Here, take this sword!".to_string(),
    choices: vec![/* ... */],
    on_enter: Some("give_sword".to_string()),
}

// Handle events
fn handle_dialogue_event(event: &str) {
    match event {
        "give_sword" => inventory.add_item(create_sword()),
        "start_quest" => quest_log.start_quest("rescue_princess"),
        "heal_player" => player.heal(player.max_health),
        _ => {}
    }
}
```

## Putting It All Together

Here's an example of a complete game scene using all systems:

```rust
use phoenix_engine::game::*;

struct GameScene {
    player: Player,
    enemies: Vec<Enemy>,
    inventory: Inventory,
    dialogue: DialogueManager,
    current_npc: Option<usize>,
}

impl GameScene {
    fn update(&mut self, delta: f32, input: &InputManager) {
        // Update player
        self.player.update(delta);
        
        // Handle input
        if !self.dialogue.is_active() {
            self.handle_player_input(input, delta);
        } else {
            self.handle_dialogue_input(input);
        }
        
        // Update enemies
        for enemy in &mut self.enemies {
            enemy.update(delta);
            
            // Chase player if seen
            if enemy.can_see_player(enemy.pos, self.player.pos) {
                enemy.state = EnemyState::Chase;
            }
        }
        
        // Combat
        self.handle_combat(delta);
    }
    
    fn handle_player_input(&mut self, input: &InputManager, delta: f32) {
        let h = input.get_axis("horizontal");
        
        // Movement
        if h.abs() > 0.1 {
            self.player.pos.x += h * self.player.move_speed * delta;
            self.player.facing_right = h > 0.0;
        }
        
        // Jump
        if input.is_action_pressed("jump") {
            self.player.buffer_jump();
        }
        
        // Attack
        if input.is_action_pressed("attack") {
            self.player.start_attack();
        }
        
        // Interact
        if input.is_action_pressed("interact") {
            self.try_interact();
        }
    }
    
    fn handle_combat(&mut self, delta: f32) {
        // Check player attacks hitting enemies
        if self.player.state == PlayerState::Attacking {
            for enemy in &mut self.enemies {
                if enemy.is_alive() && collides(attack_hitbox, enemy.hitbox) {
                    enemy.take_damage(self.player.attack_damage);
                }
            }
        }
        
        // Check enemies hitting player
        for enemy in &self.enemies {
            if enemy.state == EnemyState::Attack && !self.player.is_invincible() {
                if collides(enemy.hitbox, player.hitbox) {
                    self.player.take_damage(enemy.attack_damage);
                }
            }
        }
    }
}
```

---

[← Physics](./physics.md) | [Python Scripting →](./python-scripting.md)
