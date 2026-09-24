// libs/mvframe/src/widget/plugin_manager.rs

pub mod data;
pub use data::PluginManagerData;

mod view;

pub struct PluginManagerWidget<'a> {
    pub data: PluginManagerData<'a>,
    pub is_open: bool,
}
