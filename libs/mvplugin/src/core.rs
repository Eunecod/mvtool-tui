// libs/mvplugin/src/core.rs

pub mod plugin;
pub use plugin::Plugin;

pub mod metadata;
pub use metadata::PluginMetadata;

pub mod instance;
pub use instance::PluginInstance;

pub mod models;
pub use models::Project;

pub mod setting;
pub use setting::PluginSettings;
