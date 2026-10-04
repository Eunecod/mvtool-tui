// libs/mvplugin/src/api/execute.rs

use crate::loader::RegisterContext;
use mvcore::events::Command;

pub fn execute(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let execute = ctx
        .lua
        .create_function(move |_, (name, command): (String, String)| {
            let _ = tx.blocking_send(Command::Execute(name, command));

            Ok(())
        })
        .map_err(|error| format!("Ошибка загрузки [api: execute]: {}", error))?;

    ctx.api
        .set("execute", execute)
        .map_err(|error| error.to_string())?;

    Ok(())
}
