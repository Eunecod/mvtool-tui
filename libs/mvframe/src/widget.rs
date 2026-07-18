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
pub mod messagebox;
pub mod plugin;
pub mod plugin_manager;
pub mod plugins;
pub mod projects;
pub mod scripts;
pub mod settings;
pub mod updates;

pub use about::AboutWidget;
pub use components::ComponentsWidget;
pub use configures::ConfiguresWidget;
pub use console::ConsoleWidget;
pub use messagebox::MessageBox;
pub use plugin::PluginWidget;
pub use plugin_manager::PluginManagerWidget;
pub use plugins::ItemMenuPlugins;
pub use projects::ProjectsWidget;
pub use scripts::ScriptsWidget;
pub use settings::SettingsWidget;
pub use updates::UpdatesWidget;
