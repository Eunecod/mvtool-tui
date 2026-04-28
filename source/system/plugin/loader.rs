// source/system/plugin/loader.rs

use mlua::Lua;
use mlua::Table;

use std::fs;
use std::path::Path;
use std::collections::HashMap;

use std::sync::mpsc::Sender;

use crate::application::SharedRoot;
use crate::application::events::AppEvent;

use crate::widgets::console::MessageType;

pub struct Plugin {
    #[expect(unused)]
    lua: Lua,
    hook: Table,
    context: Table
}

impl Plugin {
    pub fn hook(&self) -> &Table {
        &self.hook
    }

    pub fn context(&self) -> &Table {
        &self.context
    }
}

pub struct Api {
    pub sender: Sender<AppEvent>,
    pub data: SharedRoot
}

pub struct PluginLoader {
    pub plugins: HashMap<String, Plugin>,
    pub api: Api,
}

impl PluginLoader {
    pub fn new(api: Api) -> Result<Self, String> {
        Ok(Self { plugins: HashMap::new(), api: api })
    }

    pub fn load_plugins(&mut self, path: &str) -> Result<(), String> {
        let entries = fs::read_dir(path).map_err(|error| format!("Ошибка чтения директории с плагинами '{}': {}", path, error))?;

        for entry in entries {
            let path = entry.map_err(|error| format!("Ошибка чтения: {}", error))?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("lua") {
                continue;
            }

            let name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| format!("Невалидное имя файла: {:?}", path))?.to_string();

            self.load_plugin(&path, &name)?;
        }

        Ok(())
    }

    fn load_plugin(&mut self, path: &Path, name: &str) -> Result<(), String> {
        let code = fs::read_to_string(path).map_err(|error| format!("Ошибка чтения файла {:?}: {}", path, error))?;

        let lua = Lua::new();

        let globals = lua.globals();
        
        let api = self.impl_api(&lua).map_err(|error| error.to_string())?;
        globals.set("api", api).map_err(|error| error.to_string())?;

        let context = lua.create_table().map_err(|error| error.to_string())?;
        context.set("plugin_name", name).map_err(|error| error.to_string())?;
        
        let table = lua.create_table().map_err(|error| error.to_string())?;
        context.set("state", table).map_err(|error| error.to_string())?;

        globals.set("context", context.clone()).map_err(|error| error.to_string())?;

        let hook = lua.load(&code).eval().map_err(|error| format!("Ошибка загрузки плагина '{}': {}", name, error))?;

        self.plugins.insert(
            name.to_string(),
            Plugin {
                lua: lua,
                hook : hook,
                context: context
            }
        );

        Ok(())
    }

    fn impl_api(&self, lua: &Lua) -> Result<Table, String> {
        let api = lua.create_table().map_err(|error| format!("Критическая ошибка внедрения api: {}", error))?;
        
        let bus = self.api.sender.clone();
        let data = self.api.data.clone();

        /* impl log */
        let log = lua.create_function(move |_, message: String| {
            let _ = bus.send(AppEvent::Devent(message, MessageType::Info));
            Ok(())
        }).map_err(|error| format!("Ошибка загрузки [api: log]: {}", error))?;

        api.set("log", log).map_err(|error| error.to_string())?;

        /* impl root */
        let root = lua.create_function(move |lua, key: String| {
            let data = data.read();

            match key.as_str() {
                "projects" => {
                    let table = lua.create_table()?;

                    for (i, project) in data.projects.iter().enumerate() {
                        table.set(i + 1, project.clone())?;
                    }

                    Ok(mlua::Value::Table(table))
                }

                _ => Ok(mlua::Value::Nil),
            }
        }).map_err(|error| format!("Ошибка загрузки [api: root]: {}", error))?;
        
        api.set("root", root).map_err(|error| error.to_string())?;

        Ok(api)
    }
}