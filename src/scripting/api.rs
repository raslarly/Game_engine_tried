//! Python API
//! 
//! Exposes game engine functionality to Python scripts.

use pyo3::prelude::*;

/// Engine module exposed to Python
#[pymodule]
fn phoenix_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyVector2>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<PyEntity>()?;
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    m.add_function(wrap_pyfunction!(log_warning, m)?)?;
    m.add_function(wrap_pyfunction!(log_error, m)?)?;
    Ok(())
}

/// 2D Vector exposed to Python
#[pyclass]
#[derive(Clone)]
pub struct PyVector2 {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
}

#[pymethods]
impl PyVector2 {
    #[new]
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    fn __repr__(&self) -> String {
        format!("Vector2({}, {})", self.x, self.y)
    }
    
    fn __add__(&self, other: &PyVector2) -> PyVector2 {
        PyVector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    fn __sub__(&self, other: &PyVector2) -> PyVector2 {
        PyVector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    
    fn __mul__(&self, scalar: f32) -> PyVector2 {
        PyVector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    fn normalized(&self) -> PyVector2 {
        let len = self.length();
        if len > 0.0001 {
            PyVector2 {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            PyVector2 { x: 0.0, y: 0.0 }
        }
    }
    
    fn dot(&self, other: &PyVector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    fn distance_to(&self, other: &PyVector2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    #[staticmethod]
    fn zero() -> PyVector2 {
        PyVector2 { x: 0.0, y: 0.0 }
    }
    
    #[staticmethod]
    fn one() -> PyVector2 {
        PyVector2 { x: 1.0, y: 1.0 }
    }
    
    #[staticmethod]
    fn up() -> PyVector2 {
        PyVector2 { x: 0.0, y: 1.0 }
    }
    
    #[staticmethod]
    fn right() -> PyVector2 {
        PyVector2 { x: 1.0, y: 0.0 }
    }
}

/// Transform exposed to Python
#[pyclass]
#[derive(Clone)]
pub struct PyTransform {
    #[pyo3(get, set)]
    pub position: PyVector2,
    #[pyo3(get, set)]
    pub rotation: f32,
    #[pyo3(get, set)]
    pub scale: PyVector2,
}

#[pymethods]
impl PyTransform {
    #[new]
    fn new() -> Self {
        Self {
            position: PyVector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: PyVector2 { x: 1.0, y: 1.0 },
        }
    }
    
    fn __repr__(&self) -> String {
        format!(
            "Transform(pos=({}, {}), rot={}, scale=({}, {}))",
            self.position.x, self.position.y,
            self.rotation,
            self.scale.x, self.scale.y
        )
    }
    
    fn translate(&mut self, offset: &PyVector2) {
        self.position.x += offset.x;
        self.position.y += offset.y;
    }
    
    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }
    
    fn forward(&self) -> PyVector2 {
        PyVector2 {
            x: self.rotation.cos(),
            y: self.rotation.sin(),
        }
    }
}

/// Entity handle for Python
#[pyclass]
#[derive(Clone)]
pub struct PyEntity {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub active: bool,
}

#[pymethods]
impl PyEntity {
    #[new]
    fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            active: true,
        }
    }
    
    fn __repr__(&self) -> String {
        format!("Entity('{}', id={})", self.name, self.id)
    }
}

/// Log an info message
#[pyfunction]
fn log_info(message: &str) {
    tracing::info!("[Python] {}", message);
}

/// Log a warning message
#[pyfunction]
fn log_warning(message: &str) {
    tracing::warn!("[Python] {}", message);
}

/// Log an error message
#[pyfunction]
fn log_error(message: &str) {
    tracing::error!("[Python] {}", message);
}

/// Helper to create the Python API module code
pub fn get_api_module_code() -> &'static str {
    r#"
"""
Phoenix Engine Python API

This module provides Python bindings for the Phoenix 2D Game Engine.
Use this to create game configurations, define behaviors, and script game logic.
"""

class Vector2:
    """2D Vector for positions, velocities, and directions."""
    
    def __init__(self, x=0.0, y=0.0):
        self.x = float(x)
        self.y = float(y)
    
    def __repr__(self):
        return f"Vector2({self.x}, {self.y})"
    
    def __add__(self, other):
        return Vector2(self.x + other.x, self.y + other.y)
    
    def __sub__(self, other):
        return Vector2(self.x - other.x, self.y - other.y)
    
    def __mul__(self, scalar):
        return Vector2(self.x * scalar, self.y * scalar)
    
    def length(self):
        return (self.x ** 2 + self.y ** 2) ** 0.5
    
    def normalized(self):
        length = self.length()
        if length > 0.0001:
            return Vector2(self.x / length, self.y / length)
        return Vector2(0, 0)
    
    def to_dict(self):
        return {"x": self.x, "y": self.y}
    
    @staticmethod
    def zero():
        return Vector2(0, 0)
    
    @staticmethod
    def one():
        return Vector2(1, 1)
    
    @staticmethod
    def up():
        return Vector2(0, 1)
    
    @staticmethod
    def right():
        return Vector2(1, 0)


class PlayerConfig:
    """Player configuration settings."""
    
    def __init__(self):
        self.max_health = 100
        self.move_speed = 200.0
        self.jump_force = 500.0
        self.attack_damage = 25
        self.attack_cooldown = 0.5
        self.invincibility_time = 1.0
    
    def to_dict(self):
        return {
            "max_health": self.max_health,
            "move_speed": self.move_speed,
            "jump_force": self.jump_force,
            "attack_damage": self.attack_damage,
            "attack_cooldown": self.attack_cooldown,
            "invincibility_time": self.invincibility_time,
        }


class EnemyConfig:
    """Enemy type configuration."""
    
    def __init__(self, name):
        self.name = name
        self.max_health = 50
        self.move_speed = 100.0
        self.attack_damage = 10
        self.attack_range = 50.0
        self.detection_range = 200.0
        self.drop_table = {}
    
    def add_drop(self, item_id, chance):
        self.drop_table[item_id] = chance
    
    def to_dict(self):
        return {
            "name": self.name,
            "max_health": self.max_health,
            "move_speed": self.move_speed,
            "attack_damage": self.attack_damage,
            "attack_range": self.attack_range,
            "detection_range": self.detection_range,
            "drop_table": self.drop_table,
        }


class ItemConfig:
    """Item configuration."""
    
    def __init__(self, name, item_type="consumable"):
        self.name = name
        self.description = ""
        self.item_type = item_type
        self.value = 0
        self.effects = {}
    
    def add_effect(self, effect_name, value):
        self.effects[effect_name] = value
    
    def to_dict(self):
        return {
            "name": self.name,
            "description": self.description,
            "item_type": self.item_type,
            "value": self.value,
            "effects": self.effects,
        }


class LevelConfig:
    """Level/scene configuration."""
    
    def __init__(self, name):
        self.name = name
        self.spawn_point = [0.0, 0.0]
        self.bounds = [[-1000.0, -1000.0], [1000.0, 1000.0]]
        self.background_music = ""
        self.enemy_spawns = []
    
    def add_enemy_spawn(self, enemy_type, x, y, respawn_time=None):
        self.enemy_spawns.append({
            "enemy_type": enemy_type,
            "position": [x, y],
            "respawn_time": respawn_time,
        })
    
    def to_dict(self):
        return {
            "name": self.name,
            "spawn_point": self.spawn_point,
            "bounds": self.bounds,
            "background_music": self.background_music,
            "enemy_spawns": self.enemy_spawns,
        }


class GameConfig:
    """Main game configuration container."""
    
    def __init__(self):
        self.player = PlayerConfig()
        self.enemies = {}
        self.items = {}
        self.levels = {}
        self.custom = {}
    
    def add_enemy(self, enemy_id, config):
        self.enemies[enemy_id] = config
    
    def add_item(self, item_id, config):
        self.items[item_id] = config
    
    def add_level(self, level_id, config):
        self.levels[level_id] = config
    
    def set_custom(self, key, value):
        self.custom[key] = value
    
    def to_dict(self):
        return {
            "player": self.player.to_dict(),
            "enemies": {k: v.to_dict() for k, v in self.enemies.items()},
            "items": {k: v.to_dict() for k, v in self.items.items()},
            "levels": {k: v.to_dict() for k, v in self.levels.items()},
            "custom": self.custom,
        }
"#
}
