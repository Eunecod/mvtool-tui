// libs/mvplugin/src/api/copying.rs

use crate::core::models::Project as LuaProject;
use crate::loader::RegisterContext;
use mvcore::events::Command;

pub fn copying(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let copying = ctx
        .lua
        .create_function(move |_, project: LuaProject| {
            let _ = tx.blocking_send(Command::Copyng(project.0.clone()));

            Ok(())
        })
        .map_err(|error| format!("Ошибка загрузки [api: copying]: {}", error))?;

    ctx.api
        .set("copying", copying)
        .map_err(|error| error.to_string())?;

    Ok(())
}
