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

// Re-export core types for convenience
// Some may not be used internally but are part of the public API
#[allow(unused_imports)]
pub use ecs::World;
#[allow(unused_imports)]
pub use transform::Transform;
#[allow(unused_imports)]
pub use renderer::Renderer;
#[allow(unused_imports)]
pub use audio::AudioManager;
#[allow(unused_imports)]
pub use input::InputManager;
#[allow(unused_imports)]
pub use resources::ResourceManager;
#[allow(unused_imports)]
pub use time::Time;
#[allow(unused_imports)]
pub use physics::PhysicsWorld;
