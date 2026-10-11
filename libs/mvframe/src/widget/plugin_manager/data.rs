// libs/mvframe/src/widget/plugins_manager/data.rs

use std::collections::HashMap;

use mvcore::events::Command;
use mvplugin::core::plugin::Plugin;

use tokio::sync::mpsc;

pub struct PluginManagerData<'a> {
    pub plugins: &'a mut HashMap<String, Plugin>,
    pub tx: mpsc::Sender<Command>,
}

impl<'a> PluginManagerData<'a> {
    pub fn new(plugins: &'a mut HashMap<String, Plugin>, tx: mpsc::Sender<Command>) -> Self {
        Self { plugins, tx }
    }
}
