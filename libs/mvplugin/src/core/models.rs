// libs/mvplugin/src/core/models.rs

use mlua::{FromLua, IntoLua, Lua, Result, Value};

#[derive(Clone)]
pub struct Root(pub mvcore::io::Root);

impl IntoLua for Root {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;

        table.set(
            "projects",
            self.0.projects.into_iter().map(Project).collect::<Vec<_>>(),
        )?;

        Ok(Value::Table(table))
    }
}

impl FromLua for Root {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        if let Value::Table(table) = value {
            let projects: Vec<Project> = table.get("projects")?;

            Ok(Root(mvcore::io::Root {
                projects: projects.into_iter().map(|project| project.0).collect(),
            }))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Root".to_string(),
                message: Some("expected a table".into()),
            })
        }
    }
}

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

impl FromLua for Project {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        if let Value::Table(table) = value {
            let configures: Vec<Configure> = table.get("configures")?;

            Ok(Project(mvcore::io::Project {
                name: table.get("name")?,
                selected: table.get("selected")?,
                destination_path: table.get("destination_path")?,
                configures: configures
                    .into_iter()
                    .map(|configure| configure.0)
                    .collect(),
            }))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Project".to_string(),
                message: Some("expected a table".into()),
            })
        }
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

impl FromLua for Configure {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        if let Value::Table(table) = value {
            let components: Vec<Component> = table.get("components")?;
            let scripts: Vec<Script> = table.get("scripts")?;

            Ok(Configure(mvcore::io::Configure {
                name: table.get("name")?,
                source_path: table.get("source_path")?,
                selected: table.get("selected")?,
                clean_destination: table.get("clean_destination")?,
                extension_mask: table.get("extension_mask")?,
                components: components
                    .into_iter()
                    .map(|component| component.0)
                    .collect(),
                scripts: scripts.into_iter().map(|script| script.0).collect(),
            }))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Configure".to_string(),
                message: Some("expected a table".into()),
            })
        }
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

impl FromLua for Component {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        if let Value::Table(table) = value {
            Ok(Component(mvcore::io::Component {
                name: table.get("name")?,
                selected: table.get("selected")?,
            }))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Component".to_string(),
                message: Some("expected a table".into()),
            })
        }
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

impl FromLua for Script {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        if let Value::Table(table) = value {
            Ok(Script(mvcore::io::Script {
                name: table.get("name")?,
                command: table.get("command")?,
            }))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Script".to_string(),
                message: Some("expected a table".into()),
            })
        }
    }
}
