// source/system/plugin/mod.rs

pub mod manager;
pub use manager::PluginManager;

pub mod loader;
pub use loader::PluginLoader;
pub use loader::Api;