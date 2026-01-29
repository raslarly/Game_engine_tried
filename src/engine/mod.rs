//! Core Engine Module
//! 
//! Contains the fundamental systems for the game engine including:
//! - Entity Component System (ECS)
//! - Transform and physics
//! - Rendering pipeline
//! - Audio management
//! - Input handling
//! - Resource management

pub mod ecs;
pub mod transform;
pub mod renderer;
pub mod audio;
pub mod input;
pub mod resources;
pub mod time;
pub mod physics;

pub use ecs::World;
pub use transform::Transform;
pub use renderer::Renderer;
pub use audio::AudioManager;
pub use input::InputManager;
pub use resources::ResourceManager;
pub use time::Time;
pub use physics::PhysicsWorld;
