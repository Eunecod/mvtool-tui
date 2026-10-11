// libs/mvplugin/src/loader/context.rs

use mlua::{Lua, Table};
use tokio::sync::mpsc::Sender;

use mvcore::events::Command;

pub struct RegisterContext<'a> {
    pub api: &'a mut Table,
    pub lua: &'a Lua,
    pub tx: Sender<Command>,
}
