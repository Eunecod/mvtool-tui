// libs/mvframe/src/widget/manager_plugin.rs

use super::Widget;

use imgui::Condition;
use imgui::StyleColor;
use imgui::Ui;
use imgui::WindowFlags;

use mvplugin::api::Plugin;

pub struct PluginManagerWidget {
    is_open: bool,
}

impl PluginManagerWidget {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }
}

impl Widget<Vec<&mut Plugin>> for PluginManagerWidget {
    fn draw(&mut self, ui: &Ui, plugins: &mut Vec<&mut Plugin>) {
        if !self.is_open {
            return;
        }

        let display_size = ui.io().display_size;
        let style = ui.push_style_color(StyleColor::WindowBg, [0.1, 0.1, 0.1, 1.0]);

        let flags = WindowFlags::NO_DOCKING | WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE;

        ui.window("Менеджер плагинов###plugin_manager_widget")
            .size(display_size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .flags(flags)
            .build(|| {
                for plugin in plugins {
                    ui.separator();

                    ui.text(format!("name: {}", &plugin.meta.name));
                    ui.text(format!("version: {}", &plugin.meta.version));
                    ui.text(format!("author: {}", &plugin.meta.author));
                    ui.text(format!("description: {}", &plugin.meta.description));

                    ui.checkbox(
                        &format!("Активировать##item_{}", plugin.meta.name),
                        &mut plugin.enabled,
                    );

                    ui.separator();
                }

                let button_width = 120.0;
                let total_buttons_width = (button_width * 2.0) + ui.clone_style().item_spacing[0];

                ui.set_cursor_pos([
                    ui.content_region_max()[0] - total_buttons_width,
                    ui.cursor_pos()[1],
                ]);

                if ui.button_with_size("ОК", [button_width, 0.0]) {
                    ui.close_current_popup();
                    self.is_open = false;
                }

                ui.same_line();

                if ui.button_with_size("Отмена", [button_width, 0.0]) {
                    ui.close_current_popup();
                    self.is_open = false;
                }
            });

        style.pop();
    }
}
