// libs/mvplugin/src/api/messagebox.rs

use crate::loader::RegisterContext;
use mvcore::events::Command;

use mlua::{Function, Lua};

use std::sync::{Arc, Mutex};

struct LuaPtr(*const Lua);
unsafe impl Send for LuaPtr {}

pub fn messagebox(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let messagebox = ctx
        .lua
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

                let _ = tx.blocking_send(Command::ShowMessageBox(title, message, registry_action));

                Ok(())
            },
        )
        .map_err(|error| format!("Ошибка загрузки [api: messagebox]: {}", error))?;

    ctx.api
        .set("messagebox", messagebox)
        .map_err(|error| error.to_string())?;

    Ok(())
}
