// libs/core/src/events.rs

use crate::io::schema::Project;
use crate::session::UpdateSession;

#[derive(PartialEq)]
pub enum Type {
    Message,
    Success,
    Warning,
    Error,
    Stdout,
}

pub enum Command {
    Devent(String, Type),
    Notify(String, String),
    ShowMessageBox(String, String, Box<dyn FnOnce() + Send + 'static>),
    Execute(
        String,
        String,
        Option<Box<dyn FnOnce(bool) + Send + 'static>>,
    ),
    UpdaterReady(UpdateSession),
    Update(),
    Copyng(Project),
    PluginManager(),
    Settings(),
    About(),
    TryUpdates(),
    Exit(),
    LuaCallback(Box<dyn FnOnce(bool) + Send + 'static>, bool),
}
