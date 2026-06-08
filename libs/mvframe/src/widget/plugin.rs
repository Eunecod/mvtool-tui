// libs/mvframe/src/widget/plugin.rs

use imgui::Condition;
use imgui::Ui;

use mvcore::io::Project;

use mvplugin::system::PluginManager;

pub struct PluginWidget {
    title: String,
}

impl PluginWidget {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl PluginWidget {
    pub fn draw(&mut self, ui: &Ui, plugin_manager: &PluginManager, projects: &mut Vec<Project>) {
        mvplugin::api::bridge::imgui::CURRENT_UI.with(|cell| cell.set(Some(ui as *const Ui)));

        let header = format!("{}###{}_plugin_window", self.title, self.title);
        ui.window(&header)
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                if let Err(error) = plugin_manager.emit("build", &projects, Some(&self.title)) {
                    ui.text_wrapped(error);
                }
            });

        mvplugin::api::bridge::imgui::CURRENT_UI.with(|cell| cell.set(None));
    }
}
