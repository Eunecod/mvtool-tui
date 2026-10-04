// libs/mvplugin/src/core/plugin.rs

use mlua::{Function, IntoLua};

use crate::core::instance::PluginInstance;
use crate::core::metadata::PluginMetadata;

pub enum PluginState {
    Disabled,
    Active(Box<PluginInstance>),
}

pub struct Plugin {
    pub metadata: PluginMetadata,
    pub state: PluginState,
    pub shortcut: Option<u32>,
}

impl Plugin {
    pub fn emit<A>(&mut self, hook: &str, args: A) -> Result<(), String>
    where
        A: IntoLua + Clone,
    {
        let instance = match &mut self.state {
            PluginState::Active(instance) => instance,
            PluginState::Disabled => return Ok(()),
        };

        if let Some(lua_function) = instance.hook(hook) {
            let lua_args = args.into_lua(&instance.lua).map_err(|error| {
                format!("Критическая ошибка конвертации аргументов в Lua обратитесь к разработчику mvtool: {}", error)
            })?;

            lua_function
                .call::<()>(lua_args)
                .map_err(|error| format!("Ошибка выполнения {}: {}", hook, error))?;
        }

        Ok(())
    }

    pub fn has_hook(&self, hook: &str) -> bool {
        match &self.state {
            PluginState::Active(instance) => {
                return instance.table.get::<Function>(hook).is_ok();
            }
            PluginState::Disabled => return false,
        };
    }
}
