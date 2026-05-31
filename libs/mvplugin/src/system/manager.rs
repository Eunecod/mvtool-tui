// libs/mvplugin/src/system/manager.rs

use mlua::Function;
use mlua::IntoLua;

use tokio::sync::mpsc;

use mvcore::events::Command;
use mvcore::io::Project;

use super::loader::PluginLoader;
use crate::api::bridge::models::Project as LuaProject;

pub struct PluginManager {
    pub loader: PluginLoader,
}

impl PluginManager {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            loader: PluginLoader::new(tx),
        }
    }

    pub fn emit(
        &self,
        hook_name: &str,
        data: &[Project],
        name_plugin: Option<&str>,
    ) -> Result<(), String> {
        for (name, plugin) in &self.loader.plugins {
            if !plugin.enabled {
                continue;
            }

            if let Some(plugin_name) = name_plugin {
                if plugin_name != name {
                    continue;
                }
            }

            let lua_table = plugin.lua.create_table().map_err(|error| {
                format!(
                    "Не удалось создать таблицу для плагина '{}': {}",
                    name, error
                )
            })?;

            for (i, project) in data.iter().enumerate() {
                let lua_project =
                    LuaProject(project.clone())
                        .into_lua(&plugin.lua)
                        .map_err(|error| {
                            format!("Ошибка конвертации проекта в плагине '{}': {}", name, error)
                        })?;

                lua_table.set(i + 1, lua_project).map_err(|error| {
                    format!(
                        "Ошибка добавления проекта в таблицу плагина '{}': {}",
                        name, error
                    )
                })?;
            }

            match plugin.hook().get::<Function>(hook_name) {
                Ok(func) => {
                    if let Err(error) = func.call::<()>((lua_table, plugin.context().clone())) {
                        return Err(format!(
                            "Ошибка выполнения функции '{}' в плагине '{}': {}",
                            hook_name, name, error
                        ));
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
