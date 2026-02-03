// RetroVaders - Space Invaders (1978) Recreation
// Library exports for testing

// Allow dead code for features not yet implemented
#![allow(dead_code)]

pub mod game;
pub mod entities;
pub mod systems;
pub mod rendering;
pub mod audio;
pub mod input;

// Re-export commonly used types
pub use game::config::*;
pub use game::state::GameState;
pub use game::world::World;
