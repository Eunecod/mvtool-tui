// libs/mvplugin/src/api/execute.rs

use crate::api::LuaCallbackGuard;
use crate::loader::RegisterContext;
use mvcore::events::Command;

use mlua::Value;

pub fn execute(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let execute = ctx
        .lua
        .create_function(
            move |lua, (name, command, callback): (String, String, Value)| {
                let lua_callback = LuaCallbackGuard::new(lua, callback)?;
                let registry_callback: Option<Box<dyn FnOnce(bool) + Send + 'static>> =
                    lua_callback.map(|guard| {
                        Box::new(move |success: bool| {
                            let _ = guard.call::<bool, ()>(success);
                        }) as Box<dyn FnOnce(bool) + Send + 'static>
                    });
                let _ = tx.blocking_send(Command::Execute(name, command, registry_callback));

                Ok(())
            },
        )
        .map_err(|error| format!("Ошибка загрузки [api: execute]: {}", error))?;

    ctx.api
        .set("execute", execute)
        .map_err(|error| error.to_string())?;

    Ok(())
}
