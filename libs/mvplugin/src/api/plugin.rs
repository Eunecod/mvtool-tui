// libs/mvplugin/src/api/plugin.rs

use mlua::Lua;
use mlua::Table;

#[derive(Default)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub requirement: String,
}

pub struct Plugin {
    pub lua: Lua,
    pub hook: Table,
    pub context: Table,
    pub meta: PluginMeta,
    pub enabled: bool,
}

impl Plugin {
    pub fn hook(&self) -> &Table {
        &self.hook
    }

    pub fn context(&self) -> &Table {
        &self.context
    }
}
