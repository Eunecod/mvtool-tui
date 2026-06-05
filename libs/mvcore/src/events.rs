// libs/core/src/events.rs

#[derive(PartialEq)]
pub enum Type {
    Message,
    Success,
    Warning,
    Error,
}

pub enum Command {
    Devent(String, Type),
    ShowNotification(String, String),
    ShowMessageBox(String, String, Box<dyn FnOnce() + Send + 'static>),
    Execute(String, String),
    UpdaterReady(crate::session::UpdateSession),
    Update(),
    Copyng(),
    PluginManager(),
    About(),
    Exit(),
}
