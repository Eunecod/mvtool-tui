// libs/mvframe/src/widget/plugins/item_menu.rs

pub mod data;
pub use data::ItemMenuData;

mod view;

use mvplugin::system::PluginManager;

pub struct ItemMenuPlugins<'a> {
    pub data: &'a mut ItemMenuData,
    pub plugin_manager: &'a mut PluginManager,
}
