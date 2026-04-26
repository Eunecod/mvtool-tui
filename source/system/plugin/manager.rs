// source/system/plugin/manager.rs

use mlua::Function;
use mlua::Table;
use mlua::Value;

use super::loader::PluginLoader;

use crate::system::plugin::Plugin;

pub struct PluginManager;

impl PluginManager {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, loader: &PluginLoader, hook_name: &str, data: &Value) -> Result<(), String> {
        for (name, plugin) in &loader.plugins {

            let _ = self.update_api(loader, plugin);

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

    fn update_api(&self, loader: &PluginLoader, plugin: &Plugin) -> Result<(), String> {
        let globals = plugin.lua().globals();
        let api = globals.get::<Table>("api").map_err(|error| error.to_string())?;

        let root = loader.api.data.clone();

        let data = plugin.lua().create_function(move |lua, key: String| {
            match key.as_str() {
                "projects" => {
                    let table = lua.create_table()?;
                    
                    for (i, project) in root.projects.iter().enumerate() {
                        let project_table = lua.create_table()?;
                        project_table.set("name", project.name.clone())?;
                        project_table.set("selected", project.selected)?;
                
                        table.set(i + 1, project_table)?;
                    }
                    
                    Ok(Value::Table(table))
                }
    
                _ => Ok(Value::Nil)
            }
        }).map_err(|error| format!("Ошибка обновления [api: data]: {}", error))?;
    
        api.set("data", data).map_err(|error| error.to_string())?;

        Ok(())
    }
}