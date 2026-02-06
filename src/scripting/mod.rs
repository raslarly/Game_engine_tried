//! Python Scripting Module
//! 
//! Provides Python integration for configuration and game logic.
//! Enable the "python" feature to use Python scripting.

#[cfg(feature = "python")]
pub mod runtime;
pub mod config;
#[cfg(feature = "python")]
pub mod api;

#[cfg(feature = "python")]
pub use runtime::PythonRuntime;
#[allow(unused_imports)]
pub use config::ConfigManager;

/// Stub Python runtime when the python feature is disabled
#[cfg(not(feature = "python"))]
pub struct PythonRuntime;

#[cfg(not(feature = "python"))]
impl PythonRuntime {
    pub fn new() -> Result<Self, String> {
        Err("Python support not compiled. Enable the 'python' feature.".to_string())
    }
    
    pub fn execute(&self, _code: &str) -> Result<(), String> {
        Err("Python not available".to_string())
    }
    
    pub fn is_available() -> bool {
        false
    }
}
