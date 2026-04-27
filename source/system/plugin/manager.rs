// source/system/plugin/manager.rs

use mlua::Function;
use mlua::Value;

use super::loader::PluginLoader;

pub struct PluginManager;

impl PluginManager {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, loader: &PluginLoader, hook_name: &str, data: &Value) -> Result<(), String> {
        for (name, plugin) in &loader.plugins {
            match plugin.hook().get::<Function>(hook_name) {
                Ok(func) => {
                    if let Err(error) = func.call::<()>((data, plugin.context().clone())) {
                       return Err(format!("Ошибка выполнения функции '{}' в плагине '{}': {}", hook_name, name, error));
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }

        Ok(())
    }
}