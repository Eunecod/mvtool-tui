// libs/mvplugin/src/bridge/models.rs

use mlua::IntoLua;
use mlua::Lua;
use mlua::Result;
use mlua::Value;

#[derive(Clone)]
pub struct Project(pub mvcore::io::Project);

impl IntoLua for Project {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.0.name)?;
        table.set("selected", self.0.selected)?;
        table.set("destination_path", self.0.destination_path)?;

        table.set(
            "configures",
            self.0
                .configures
                .into_iter()
                .map(Configure)
                .collect::<Vec<_>>(),
        )?;

        Ok(Value::Table(table))
    }
}

#[derive(Clone)]
pub struct Configure(pub mvcore::io::Configure);

impl IntoLua for Configure {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.0.name)?;
        table.set("source_path", self.0.source_path)?;
        table.set("selected", self.0.selected)?;
        table.set("clean_destination", self.0.clean_destination)?;
        table.set("extension_mask", self.0.extension_mask)?;

        table.set(
            "components",
            self.0
                .components
                .into_iter()
                .map(Component)
                .collect::<Vec<_>>(),
        )?;

        table.set(
            "scripts",
            self.0.scripts.into_iter().map(Script).collect::<Vec<_>>(),
        )?;

        Ok(Value::Table(table))
    }
}

#[derive(Clone)]
pub struct Component(pub mvcore::io::Component);

impl IntoLua for Component {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.0.name)?;
        table.set("selected", self.0.selected)?;

        Ok(Value::Table(table))
    }
}

#[derive(Clone)]
pub struct Script(pub mvcore::io::Script);

impl IntoLua for Script {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set("name", self.0.name)?;
        table.set("command", self.0.command)?;

        Ok(Value::Table(table))
    }
}
