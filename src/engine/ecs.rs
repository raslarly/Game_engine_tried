//! Entity Component System (ECS)
//! 
//! Provides a simple and efficient way to manage game entities and their components.

use hecs::{Entity, World as HecsWorld};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A unique identifier for entities that can be serialized
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

/// Game world containing all entities and systems
pub struct World {
    /// The underlying ECS world
    pub ecs: HecsWorld,
    /// Mapping from serializable IDs to ECS entities
    id_map: HashMap<EntityId, Entity>,
    /// Reverse mapping for lookups
    entity_to_id: HashMap<Entity, EntityId>,
    /// Named entities for easy access
    named_entities: HashMap<String, EntityId>,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        Self {
            ecs: HecsWorld::new(),
            id_map: HashMap::new(),
            entity_to_id: HashMap::new(),
            named_entities: HashMap::new(),
        }
    }
    
    /// Spawn a new entity with components
    pub fn spawn<C: hecs::DynamicBundle>(&mut self, components: C) -> EntityId {
        let entity = self.ecs.spawn(components);
        let id = EntityId::new();
        self.id_map.insert(id, entity);
        self.entity_to_id.insert(entity, id);
        id
    }
    
    /// Spawn a named entity
    pub fn spawn_named<C: hecs::DynamicBundle>(&mut self, name: &str, components: C) -> EntityId {
        let id = self.spawn(components);
        self.named_entities.insert(name.to_string(), id);
        id
    }
    
    /// Get entity by its ID
    pub fn get_entity(&self, id: EntityId) -> Option<Entity> {
        self.id_map.get(&id).copied()
    }
    
    /// Get entity by name
    pub fn get_named(&self, name: &str) -> Option<EntityId> {
        self.named_entities.get(name).copied()
    }
    
    /// Get the ID for an entity
    pub fn get_id(&self, entity: Entity) -> Option<EntityId> {
        self.entity_to_id.get(&entity).copied()
    }
    
    /// Despawn an entity
    pub fn despawn(&mut self, id: EntityId) -> bool {
        if let Some(entity) = self.id_map.remove(&id) {
            self.entity_to_id.remove(&entity);
            // Remove from named entities if present
            self.named_entities.retain(|_, v| *v != id);
            self.ecs.despawn(entity).is_ok()
        } else {
            false
        }
    }
    
    /// Get a component from an entity
    pub fn get<T: hecs::Component>(&self, id: EntityId) -> Option<hecs::Ref<T>> {
        let entity = self.id_map.get(&id)?;
        self.ecs.get::<&T>(*entity).ok()
    }
    
    /// Get a mutable component from an entity
    pub fn get_mut<T: hecs::Component>(&self, id: EntityId) -> Option<hecs::RefMut<T>> {
        let entity = self.id_map.get(&id)?;
        self.ecs.get::<&mut T>(*entity).ok()
    }
    
    /// Add a component to an entity
    pub fn add_component<T: hecs::Component>(&mut self, id: EntityId, component: T) -> bool {
        if let Some(&entity) = self.id_map.get(&id) {
            self.ecs.insert_one(entity, component).is_ok()
        } else {
            false
        }
    }
    
    /// Remove a component from an entity
    pub fn remove_component<T: hecs::Component>(&mut self, id: EntityId) -> Option<T> {
        let entity = self.id_map.get(&id)?;
        self.ecs.remove_one::<T>(*entity).ok()
    }
    
    /// Get the count of entities
    pub fn entity_count(&self) -> usize {
        self.id_map.len()
    }
    
    /// Clear all entities
    pub fn clear(&mut self) {
        self.ecs.clear();
        self.id_map.clear();
        self.entity_to_id.clear();
        self.named_entities.clear();
    }
    
    /// Iterate over all entity IDs
    pub fn iter_ids(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.id_map.keys().copied()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Common component: Name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(pub String);

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name(s.to_string())
    }
}

/// Common component: Tag for grouping entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag(pub String);

/// Common component: Active state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Active(pub bool);

impl Active {
    pub fn enabled() -> Self {
        Self(true)
    }
    
    pub fn disabled() -> Self {
        Self(false)
    }
}

/// Common component: Layer for rendering order
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Layer(pub i32);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spawn_and_get() {
        let mut world = World::new();
        let id = world.spawn((Name::from("Test"), Layer(5)));
        
        assert!(world.get::<Name>(id).is_some());
        assert_eq!(world.get::<Layer>(id).unwrap().0, 5);
    }
    
    #[test]
    fn test_named_entity() {
        let mut world = World::new();
        let id = world.spawn_named("player", (Name::from("Player"),));
        
        assert_eq!(world.get_named("player"), Some(id));
    }
}
