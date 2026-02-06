//! Platformer Demo
//! 
//! A sample game demonstrating Phoenix Engine's capabilities for 2D platformer games.
//! This demo includes:
//! - Player movement with jump, double jump, and dash
//! - Physics and collision detection
//! - Animated sprites
//! - Enemy AI with patrol and chase behaviors
//! - Combat system with health and damage
//! - Collectible items

use glam::Vec2;

pub mod demo_game;
pub mod demo_player;
pub mod demo_enemy;
pub mod demo_level;

pub use demo_game::DemoGame;
