// libs/mvplugin/src/api/devent.rs

use crate::loader::RegisterContext;
use mvcore::events::{Command, Type};

pub fn devent(ctx: &mut RegisterContext) -> Result<(), String> {
    let tx = ctx.tx.clone();
    let devent = ctx
        .lua
        .create_function(move |_, (message, message_type_str): (String, String)| {
            let message_type = match message_type_str.as_str() {
                "Error" => Type::Error,
                "Warning" => Type::Warning,
                "Success" => Type::Success,
                "Message" | _ => Type::Message,
            };

            let _ = tx.blocking_send(Command::Devent(message, message_type));

            Ok(())
        })
        .map_err(|error| format!("Ошибка загрузки [api: devent]: {}", error))?;

    ctx.api
        .set("devent", devent)
        .map_err(|error| error.to_string())?;

    Ok(())
}
