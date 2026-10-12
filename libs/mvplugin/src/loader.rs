// libs/mvplugin/src/loader.rs

pub mod context;
pub use context::RegisterContext;

use mlua::Lua;
use mvcore::events::Type;

use std::collections::HashMap;
use std::path::Path;
use tokio::sync::mpsc::Sender;

use mvcore::events::Command;

use crate::api;

use crate::core::instance::PluginInstance;
use crate::core::metadata::PluginMetadata;
use crate::core::plugin::Plugin;
use crate::core::plugin::PluginState;

pub struct Loader {
    tx: Sender<Command>,
}

impl Loader {
    pub fn new(tx: Sender<Command>) -> Self {
        Self { tx }
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<HashMap<String, Plugin>, String> {
        let path = path.as_ref();

        std::fs::create_dir_all(path).map_err(|error| {
            format!(
                "Не удалось создать директорию '{}': {}",
                path.to_string_lossy(),
                error
            )
        })?;

        let entries = std::fs::read_dir(path).map_err(|error| {
            format!(
                "Ошибка чтения директории с плагинами '{}': {}",
                path.to_string_lossy(),
                error
            )
        })?;

        let mut result = HashMap::new();
        for entry in entries {
            let path = entry
                .map_err(|error| format!("Ошибка чтения файла плагина: {}", error))?
                .path();

            if path.extension().and_then(|extension| extension.to_str()) != Some("lua") {
                continue;
            }

            let file_name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("[unknown]");
            let file_content = std::fs::read_to_string(&path).map_err(|error| {
                format!("Ошибка чтения файла с плагином '{}': {}", file_name, error)
            })?;

            let metadata = self.load_metadata(&file_content);
            if metadata.requirement.is_empty() {
                let _ = self.tx.blocking_send(Command::Devent(format!(
                    "У плагина '{}' отсутствует требование к версии, попробуйте обновить плагин",
                    metadata.name
                ), Type::Warning));
                continue;
            }

            if Loader::is_support_version(&metadata.requirement) {
                let _ = self.tx.blocking_send(Command::Devent(format!(
                    "Для загрузки плагина '{}' требуется версия {}. Текущая версия не поддерживается",
                    metadata.name, metadata.requirement
                ), Type::Warning));
                continue;
            }

            let instance = self.load_instance(path.clone())?;

            let plugin = Plugin {
                metadata: metadata,
                state: PluginState::Active(Box::new(instance)),
                shortcut: None,
            };
            result.insert(file_name.to_string(), plugin);
        }

        Ok(result)
    }

    pub fn load_instance(&self, path: impl AsRef<Path>) -> Result<PluginInstance, String> {
        let path = path.as_ref();

        let file_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("[unknown]");
        let file_content = std::fs::read_to_string(&path).map_err(|error| {
            format!("Ошибка чтения файла с плагином '{}': {}", file_name, error)
        })?;

        let lua = Lua::new();
        let globals = lua.globals();

        let mut api = lua.create_table().map_err(|error| {
            format!(
                "Критическая ошибка создания api обратитесь к разработчику mvtool: {}",
                error
            )
        })?;

        self.register_all(&mut RegisterContext {
            api: &mut api,
            lua: &lua,
            tx: self.tx.clone(),
        })?;

        globals.set("api", api).map_err(|error| {
            format!(
                "Критическая ошибка внедрения api обратитесь к разработчику mvtool: {}",
                error
            )
        })?;

        let table = lua
            .load(&file_content)
            .eval()
            .map_err(|error| format!("Ошибка загрузки плагина '{}': {}", file_name, error))?;

        Ok(PluginInstance::new(lua, table))
    }

    fn register_all(&self, register_context: &mut RegisterContext) -> Result<(), String> {
        api::devent(register_context)?;
        api::execute(register_context)?;
        api::messagebox(register_context)?;
        api::notify(register_context)?;
        api::imgui(register_context)?;
        api::copying(register_context)?;

        Ok(())
    }

    fn is_support_version(target_version: &str) -> bool {
        mvcore::service::is_newer_version(target_version)
    }

    fn load_metadata(&self, file_content: &str) -> PluginMetadata {
        let mut metadata = PluginMetadata::default();
        for line in file_content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if !trimmed.starts_with("--") {
                break;
            }

            if let Some(index) = trimmed.find('@') {
                if let Some((tag, value)) = trimmed[index..].split_once(' ') {
                    let clean_value = value.trim().to_string();
                    match tag {
                        "@name" => metadata.name = clean_value,
                        "@version" => metadata.version = clean_value,
                        "@author" => metadata.author = clean_value,
                        "@description" => metadata.description = clean_value,
                        "@requirement" => metadata.requirement = clean_value,
                        _ => {}
                    }
                }
            }
        }

        metadata
    }
}
