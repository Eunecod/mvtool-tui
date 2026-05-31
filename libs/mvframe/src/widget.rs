// mvframe/src/widget.rs

use imgui::Ui;

pub trait Widget<TData> {
    fn draw(&mut self, ui: &Ui, data: &mut TData);
}

pub mod controls;

pub mod about;
pub mod components;
pub mod configures;
pub mod console;
pub mod plugin;
pub mod plugin_manager;
pub mod projects;
pub mod scripts;
pub mod tabmenu;
pub mod updates;

pub use about::AboutWidget;
pub use components::ComponentsWidget;
pub use configures::ConfiguresWidget;
pub use console::ConsoleWidget;
pub use plugin::PluginWidget;
pub use plugin_manager::PluginManagerWidget;
pub use projects::ProjectsWidget;
pub use scripts::ScriptsWidget;
pub use tabmenu::TabmenuWidget;
pub use updates::UpdatesWidget;
