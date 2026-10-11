// libs/mvframe/src/widget/plugin/data.rs

use mvplugin::core::plugin::Plugin;

pub struct PluginData<'a> {
    pub plugin: &'a mut Plugin,
}

impl<'a> PluginData<'a> {
    pub fn new(plugin: &'a mut Plugin) -> Self {
        Self { plugin }
    }
}
