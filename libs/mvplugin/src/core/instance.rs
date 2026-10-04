// libs/mvplugin/src/core/instance.rs

use mlua::{Function, Lua, Table};

pub struct PluginInstance {
    pub lua: Lua,
    pub table: Table,
}

impl PluginInstance {
    pub fn new(lua: Lua, table: Table) -> Self {
        Self { lua, table }
    }

    pub fn hook(&self, hook: &str) -> Option<Function> {
        self.table.get::<Function>(hook).ok()
    }
}
