// source/system/plugin/bridge/mod.rs

use mlua::Lua;
use mlua::Value;
use mlua::Result;
use mlua::IntoLua;

use crate::models::Project;
use crate::models::Configure;
use crate::models::Component;
use crate::models::Script;

impl IntoLua for Project {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.name)?;
        table.set("selected", self.selected)?;
        table.set("destination_path", self.destination_path)?;

        table.set("configures", self.configures)?;

        Ok(Value::Table(table))
    }
}

impl IntoLua for Configure {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.name)?;
        table.set("source_path", self.source_path)?;
        table.set("selected", self.selected)?;
        table.set("clean_destination", self.clean_destination)?;
        table.set("extension_mask", self.extension_mask)?;

        table.set("components", self.components)?;
        table.set("scripts", self.scripts)?;

        Ok(Value::Table(table))
    }
}

impl IntoLua for Component {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.name)?;
        table.set("selected", self.selected)?;

        Ok(Value::Table(table))
    }
}

impl IntoLua for Script {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.name)?;
        table.set("command", self.command)?;

        Ok(Value::Table(table))
    }
}