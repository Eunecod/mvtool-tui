// libs/mvplugin/src/api.rs

use mlua::{Function, Lua, RegistryKey, Value};

pub mod devent;
pub use devent::devent;

pub mod execute;
pub use execute::execute;

pub mod messagebox;
pub use messagebox::messagebox;

pub mod notify;
pub use notify::notify;

pub mod imgui;
pub use imgui::imgui;

pub mod copying;
pub use copying::copying;

pub struct LuaCallbackGuard {
    lua_ptr: *const Lua,
    registry_key: RegistryKey,
}

unsafe impl Send for LuaCallbackGuard {}

impl LuaCallbackGuard {
    pub fn new(lua: &Lua, callback: Value) -> mlua::Result<Option<Self>> {
        let registry_key = match callback {
            Value::Function(func) => lua.create_registry_value(func)?,
            _ => return Ok(None),
        };

        Ok(Some(Self {
            lua_ptr: lua as *const Lua,
            registry_key,
        }))
    }

    pub fn call<A: mlua::IntoLuaMulti, R: mlua::FromLuaMulti>(self, args: A) -> mlua::Result<R> {
        let lua_ref = unsafe { &*self.lua_ptr };

        let func: Function = lua_ref.registry_value(&self.registry_key)?;
        let result = func.call(args);

        let _ = lua_ref.remove_registry_value(self.registry_key);

        result
    }
}
