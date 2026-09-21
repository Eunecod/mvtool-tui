// libs/mvframe/src/widget/plugins_manager/data.rs

use mvplugin::api::Plugin;

pub struct PluginManagerData<'a> {
    pub plugins: Vec<&'a mut Plugin>,
}

impl<'a> PluginManagerData<'a> {
    pub fn new(plugins: Vec<&'a mut Plugin>) -> Self {
        Self { plugins }
    }
}
