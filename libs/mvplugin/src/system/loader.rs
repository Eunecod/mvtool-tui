// libs/mvplugin/src/system/loader.rs

use mlua::Function;
use mlua::Lua;
use mlua::Table;

use tokio::sync::mpsc;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use mvcore::events::Command;
use mvcore::events::Type;

use crate::Api;
use crate::api::Plugin;
use crate::api::PluginMeta;
use crate::api::bridge::imgui::with_ui;

struct LuaPtr(*const Lua);
unsafe impl Send for LuaPtr {}

pub struct PluginLoader {
    pub plugins: HashMap<String, Plugin>,
    pub api: Api,
}

impl PluginLoader {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            plugins: HashMap::new(),
            api: Api { tx },
        }
    }

    pub fn load_plugins(&mut self, path: &str) -> Result<(), String> {
        let entries = std::fs::read_dir(path).map_err(|error| {
            format!("Ошибка чтения директории с плагинами '{}': {}", path, error)
        })?;

        for entry in entries {
            let path = entry
                .map_err(|error| format!("Ошибка чтения: {}", error))?
                .path();

            if path.extension().and_then(|s| s.to_str()) != Some("lua") {
                continue;
            }

            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| format!("Невалидное имя файла: {:?}", path))?
                .to_string();

            self.load(&path, &name)?;
        }

        Ok(())
    }

    fn load(&mut self, path: &Path, name: &str) -> Result<(), String> {
        let code = std::fs::read_to_string(path)
            .map_err(|error| format!("Ошибка чтения файла {:?}: {}", path, error))?;

        let meta = self.load_meta(&code);

        if meta.requirement.is_empty() {
            return Err(format!(
                "У плагина '{}' отсутствует требование к версии, попробуйте обновить плагин",
                name
            ));
        }

        if PluginLoader::is_support_version(&meta.requirement) {
            return Err(format!(
                "Плагин '{}' требует версию {}, которая не поддерживается текущей системой",
                name, meta.requirement
            ));
        }

        let lua = Lua::new();
        let globals = lua.globals();

        let api = self.impl_api(&lua).map_err(|error| error.to_string())?;
        globals.set("api", api).map_err(|error| error.to_string())?;

        let context = lua.create_table().map_err(|error| error.to_string())?;
        context
            .set("plugin_name", name)
            .map_err(|error| error.to_string())?;

        let table = lua.create_table().map_err(|error| error.to_string())?;
        context
            .set("state", table)
            .map_err(|error| error.to_string())?;

        globals
            .set("context", context.clone())
            .map_err(|error| error.to_string())?;

        let hook = lua
            .load(&code)
            .eval()
            .map_err(|error| format!("Ошибка загрузки плагина '{}': {}", name, error))?;

        self.plugins.insert(
            meta.name.to_string(),
            Plugin {
                lua: lua,
                hook: hook,
                context: context,
                meta: meta,
                enabled: true,
            },
        );

        Ok(())
    }

    fn is_support_version(target_version: &str) -> bool {
        mvcore::service::is_newer_version(target_version)
    }

    fn load_meta(&mut self, code: &String) -> PluginMeta {
        let mut meta = PluginMeta::default();
        for line in code.lines() {
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
                        "@name" => meta.name = clean_value,
                        "@version" => meta.version = clean_value,
                        "@author" => meta.author = clean_value,
                        "@description" => meta.description = clean_value,
                        "@requirement" => meta.requirement = clean_value,
                        _ => {}
                    }
                }
            }
        }

        meta
    }

    fn impl_api(&self, lua: &Lua) -> Result<Table, String> {
        let api = lua.create_table().map_err(|error| {
            format!(
                "Критическая ошибка внедрения api обратитесь к разработчику mvtool: {}",
                error
            )
        })?;

        /* impl version */
        let version = lua
            .create_function(move |_, ()| Ok(mvcore::service::version()))
            .map_err(|error| format!("Ошибка загрузки [api: execute]: {}", error))?;

        api.set("version", version)
            .map_err(|error| error.to_string())?;

        let tx = self.api.tx.clone();

        /* impl devent */
        let devent = lua
            .create_function(move |_, message: String| {
                let _ = tx.blocking_send(Command::Devent(message, Type::Message));

                Ok(())
            })
            .map_err(|error| format!("Ошибка загрузки [api: devent]: {}", error))?;

        api.set("devent", devent)
            .map_err(|error| error.to_string())?;

        let tx = self.api.tx.clone();

        /* impl executor */
        let execute = lua
            .create_function(move |_, (name, command): (String, String)| {
                let _ = tx.blocking_send(Command::Execute(name, command));

                Ok(())
            })
            .map_err(|error| format!("Ошибка загрузки [api: execute]: {}", error))?;

        api.set("execute", execute)
            .map_err(|error| error.to_string())?;

        let tx = self.api.tx.clone();

        /* impl message box */
        let messagebox = lua
            .create_function(
                move |lua, (title, message, action): (String, String, Function)| {
                    let registry_key = lua.create_registry_value(action)?;

                    let shared_key = Arc::new(Mutex::new(Some(registry_key)));
                    let shared_key_for_action = shared_key.clone();

                    let lua_wrapper = LuaPtr(lua as *const Lua);

                    let registry_action = Box::new(move || {
                        let wrapper = lua_wrapper;
                        let lua_ref = unsafe { &*wrapper.0 };

                        if let Ok(mut guard) = shared_key_for_action.lock() {
                            if let Some(key) = guard.take() {
                                if let Ok(lua_func) = lua_ref.registry_value::<Function>(&key) {
                                    let _ = lua_func.call::<()>(());
                                }

                                let _ = lua_ref.remove_registry_value(key);
                            }
                        }
                    });

                    let _ =
                        tx.blocking_send(Command::ShowMessageBox(title, message, registry_action));

                    Ok(())
                },
            )
            .map_err(|error| format!("Ошибка загрузки [api: messagebox]: {}", error))?;

        api.set("messagebox", messagebox)
            .map_err(|error| error.to_string())?;

        /* impl imgui */
        let imgui = lua.create_table().map_err(|error| {
            format!(
                "Критическая ошибка внедрения api обратитесь к разработчику mvtool: {}",
                error
            )
        })?;

        let imgui_text = lua
            .create_function(|_, text: String| unsafe {
                with_ui(|ui| {
                    ui.text(text);
                    Ok(())
                })
            })
            .map_err(|error| format!("Ошибка загрузки [api: imgui_text]: {}", error))?;

        imgui
            .set("text", imgui_text)
            .map_err(|error| error.to_string())?;

        let imgui_button = lua
            .create_function(|_, text: String| unsafe {
                with_ui(|ui| {
                    let clicked = ui.button(text);
                    Ok(clicked)
                })
            })
            .map_err(|error| format!("Ошибка загрузки [api: imgui_button]: {}", error))?;

        imgui
            .set("button", imgui_button)
            .map_err(|error| error.to_string())?;

        let imgui_menu_item = lua
            .create_function(|_, item_name: String| unsafe {
                with_ui(|ui| {
                    let clicked = ui.menu_item(item_name);
                    Ok(clicked)
                })
            })
            .map_err(|error| format!("Ошибка загрузки [api: imgui_menu_item]: {}", error))?;

        imgui
            .set("menu_item", imgui_menu_item)
            .map_err(|error| error.to_string())?;

        api.set("imgui", imgui).map_err(|error| error.to_string())?;

        Ok(api)
    }
}
