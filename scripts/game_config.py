"""
Phoenix Engine - Game Configuration Script

This script defines the game configuration using Python.
Edit this file to customize player stats, enemies, items, and levels.
"""

def get_config():
    """Return the complete game configuration as a dictionary."""
    
    config = {
        # Player Configuration
        "player": {
            "max_health": 100,
            "move_speed": 200.0,
            "jump_force": 500.0,
            "attack_damage": 25,
            "attack_cooldown": 0.5,
            "invincibility_time": 1.0,
        },
        
        # Enemy Definitions
        "enemies": {
            "slime": {
                "name": "Slime",
                "max_health": 30,
                "move_speed": 50.0,
                "attack_damage": 5,
                "attack_range": 30.0,
                "detection_range": 150.0,
                "drop_table": {
                    "gold_coin": 0.8,
                    "slime_gel": 0.5,
                }
            },
            "skeleton": {
                "name": "Skeleton Warrior",
                "max_health": 60,
                "move_speed": 80.0,
                "attack_damage": 15,
                "attack_range": 60.0,
                "detection_range": 200.0,
                "drop_table": {
                    "gold_coin": 1.0,
                    "bone": 0.7,
                    "rusty_sword": 0.1,
                }
            },
            "boss_dragon": {
                "name": "Ancient Dragon",
                "max_health": 500,
                "move_speed": 120.0,
                "attack_damage": 40,
                "attack_range": 150.0,
                "detection_range": 400.0,
                "drop_table": {
                    "dragon_scale": 1.0,
                    "legendary_gem": 0.3,
                }
            },
        },
        
        # Item Definitions
        "items": {
            "health_potion": {
                "name": "Health Potion",
                "description": "Restores 50 HP",
                "item_type": "consumable",
                "value": 25,
                "effects": {
                    "heal": 50,
                }
            },
            "mana_potion": {
                "name": "Mana Potion",
                "description": "Restores 30 MP",
                "item_type": "consumable",
                "value": 30,
                "effects": {
                    "restore_mana": 30,
                }
            },
            "iron_sword": {
                "name": "Iron Sword",
                "description": "A sturdy iron blade",
                "item_type": "weapon",
                "value": 100,
                "effects": {
                    "attack_bonus": 10,
                }
            },
            "leather_armor": {
                "name": "Leather Armor",
                "description": "Basic protection",
                "item_type": "armor",
                "value": 75,
                "effects": {
                    "defense_bonus": 5,
                }
            },
        },
        
        # Level Definitions
        "levels": {
            "forest_start": {
                "name": "Enchanted Forest",
                "spawn_point": [0.0, 0.0],
                "bounds": [[-1000.0, -500.0], [2000.0, 1000.0]],
                "background_music": "forest_theme",
                "enemy_spawns": [
                    {"enemy_type": "slime", "position": [200.0, 0.0], "respawn_time": 30.0},
                    {"enemy_type": "slime", "position": [400.0, 0.0], "respawn_time": 30.0},
                    {"enemy_type": "slime", "position": [600.0, 0.0], "respawn_time": 30.0},
                ]
            },
            "dungeon_entrance": {
                "name": "Dark Dungeon",
                "spawn_point": [-50.0, 0.0],
                "bounds": [[-500.0, -1000.0], [500.0, 0.0]],
                "background_music": "dungeon_theme",
                "enemy_spawns": [
                    {"enemy_type": "skeleton", "position": [100.0, -200.0], "respawn_time": 60.0},
                    {"enemy_type": "skeleton", "position": [300.0, -400.0], "respawn_time": 60.0},
                ]
            },
            "boss_arena": {
                "name": "Dragon's Lair",
                "spawn_point": [0.0, 0.0],
                "bounds": [[-300.0, -200.0], [300.0, 200.0]],
                "background_music": "boss_theme",
                "enemy_spawns": [
                    {"enemy_type": "boss_dragon", "position": [0.0, 50.0], "respawn_time": None},
                ]
            },
        },
        
        # Custom Configuration Values
        "custom": {
            "game_title": "Phoenix Adventure",
            "version": "1.0.0",
            "difficulty_multiplier": 1.0,
            "enable_debug": True,
            "default_language": "en",
        }
    }
    
    return config


# Utility functions for configuration manipulation

def scale_difficulty(config, multiplier):
    """Scale enemy stats by a difficulty multiplier."""
    for enemy in config["enemies"].values():
        enemy["max_health"] = int(enemy["max_health"] * multiplier)
        enemy["attack_damage"] = int(enemy["attack_damage"] * multiplier)
    return config


def add_enemy(config, enemy_id, name, health, speed, damage, attack_range, detection_range):
    """Add a new enemy type to the configuration."""
    config["enemies"][enemy_id] = {
        "name": name,
        "max_health": health,
        "move_speed": speed,
        "attack_damage": damage,
        "attack_range": attack_range,
        "detection_range": detection_range,
        "drop_table": {}
    }
    return config


def add_item(config, item_id, name, description, item_type, value, effects=None):
    """Add a new item to the configuration."""
    config["items"][item_id] = {
        "name": name,
        "description": description,
        "item_type": item_type,
        "value": value,
        "effects": effects or {}
    }
    return config


if __name__ == "__main__":
    # Test the configuration
    import json
    config = get_config()
    print(json.dumps(config, indent=2))
