// libs/mvplugin/src/api/messagebox.rs

use crate::api::LuaCallbackGuard;
use crate::loader::RegisterContext;
use mvcore::events::Command;

use mlua::Value;

pub fn messagebox(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let messagebox = ctx
        .lua
        .create_function(
            move |lua, (title, message, callback): (String, String, Value)| {
                let lua_callback = LuaCallbackGuard::new(lua, callback)?;
                let registry_callback: Option<Box<dyn FnOnce() + Send + 'static>> = lua_callback
                    .map(|guard| {
                        Box::new(move || {
                            let _ = guard.call::<(), ()>(());
                        }) as Box<dyn FnOnce() + Send + 'static>
                    });

                if let Some(registry_callback) = registry_callback {
                    let _ = tx.blocking_send(Command::ShowMessageBox(
                        title,
                        message,
                        registry_callback,
                    ));
                }

                Ok(())
            },
        )
        .map_err(|error| format!("Ошибка загрузки [api: messagebox]: {}", error))?;

    ctx.api
        .set("messagebox", messagebox)
        .map_err(|error| error.to_string())?;

    Ok(())
}
