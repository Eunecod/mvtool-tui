// libs/mvplugin/src/api/notify.rs

use crate::loader::RegisterContext;
use mvcore::events::Command;

pub fn notify(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let notify = ctx
        .lua
        .create_function(move |_, (title, message): (String, String)| {
            let _ = tx.blocking_send(Command::Notify(title, message));

            Ok(())
        })
        .map_err(|error| format!("Ошибка загрузки [api: notify]: {}", error))?;

    ctx.api
        .set("notify", notify)
        .map_err(|error| error.to_string())?;

    Ok(())
}
