"""
Phoenix Engine - Level Editor Script

Use this script to programmatically create and modify levels.
"""

class Vector2:
    def __init__(self, x=0.0, y=0.0):
        self.x = float(x)
        self.y = float(y)
    
    def to_list(self):
        return [self.x, self.y]


class Entity:
    """Base class for level entities."""
    
    def __init__(self, name, x=0.0, y=0.0):
        self.name = name
        self.position = Vector2(x, y)
        self.rotation = 0.0
        self.scale = Vector2(1.0, 1.0)
        self.components = {}
    
    def add_component(self, component_type, data):
        self.components[component_type] = data
        return self
    
    def to_dict(self):
        return {
            "name": self.name,
            "position": self.position.to_list(),
            "rotation": self.rotation,
            "scale": self.scale.to_list(),
            "components": self.components
        }


class Sprite(Entity):
    """Entity with a sprite renderer."""
    
    def __init__(self, name, texture_id, x=0.0, y=0.0):
        super().__init__(name, x, y)
        self.add_component("Sprite", {
            "texture_id": texture_id,
            "color": [1.0, 1.0, 1.0, 1.0],
            "flip_x": False,
            "flip_y": False
        })


class Platform(Entity):
    """Static platform entity."""
    
    def __init__(self, name, x, y, width, height):
        super().__init__(name, x, y)
        self.scale = Vector2(width, height)
        self.add_component("Collider", {
            "type": "box",
            "width": width,
            "height": height,
            "is_trigger": False
        })
        self.add_component("Sprite", {
            "texture_id": "platform",
            "color": [0.6, 0.4, 0.2, 1.0]
        })


class EnemySpawner(Entity):
    """Enemy spawn point."""
    
    def __init__(self, name, enemy_type, x, y, respawn_time=30.0):
        super().__init__(name, x, y)
        self.add_component("EnemySpawner", {
            "enemy_type": enemy_type,
            "respawn_time": respawn_time,
            "max_alive": 1
        })


class Trigger(Entity):
    """Trigger zone for events."""
    
    def __init__(self, name, x, y, width, height, event_id):
        super().__init__(name, x, y)
        self.add_component("Collider", {
            "type": "box",
            "width": width,
            "height": height,
            "is_trigger": True
        })
        self.add_component("TriggerEvent", {
            "event_id": event_id,
            "one_shot": False
        })


class Level:
    """Level container for serialization."""
    
    def __init__(self, name):
        self.name = name
        self.entities = []
        self.spawn_point = Vector2(0, 0)
        self.bounds = [Vector2(-1000, -1000), Vector2(1000, 1000)]
        self.background_color = [0.1, 0.1, 0.15]
        self.music = ""
    
    def add_entity(self, entity):
        self.entities.append(entity)
        return self
    
    def set_spawn(self, x, y):
        self.spawn_point = Vector2(x, y)
        return self
    
    def set_bounds(self, min_x, min_y, max_x, max_y):
        self.bounds = [Vector2(min_x, min_y), Vector2(max_x, max_y)]
        return self
    
    def set_music(self, music_id):
        self.music = music_id
        return self
    
    def to_dict(self):
        return {
            "name": self.name,
            "spawn_point": self.spawn_point.to_list(),
            "bounds": [b.to_list() for b in self.bounds],
            "background_color": self.background_color,
            "music": self.music,
            "entities": [e.to_dict() for e in self.entities]
        }


def create_tutorial_level():
    """Create a tutorial level."""
    level = Level("Tutorial")
    level.set_spawn(0, 50)
    level.set_bounds(-200, -100, 1000, 500)
    level.set_music("tutorial_theme")
    
    # Ground
    level.add_entity(Platform("Ground", 400, -25, 1200, 50))
    
    # Platforms
    level.add_entity(Platform("Platform1", 200, 100, 150, 20))
    level.add_entity(Platform("Platform2", 500, 150, 150, 20))
    level.add_entity(Platform("Platform3", 800, 200, 150, 20))
    
    # Training dummy
    level.add_entity(
        Entity("TrainingDummy", 300, 50)
        .add_component("Sprite", {"texture_id": "dummy"})
        .add_component("Health", {"max_health": 9999, "invincible": True})
    )
    
    # Level exit trigger
    level.add_entity(Trigger("LevelExit", 950, 50, 50, 100, "load_next_level"))
    
    return level


if __name__ == "__main__":
    import json
    tutorial = create_tutorial_level()
    print(json.dumps(tutorial.to_dict(), indent=2))
