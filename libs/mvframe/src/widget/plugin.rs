// libs/mvframe/src/widget/plugin.rs

pub mod data;
pub use data::PluginData;

mod view;

pub struct PluginWidget<'a> {
    pub data: PluginData<'a>,
}
