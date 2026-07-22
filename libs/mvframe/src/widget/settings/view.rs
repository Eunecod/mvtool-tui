// libs/mvframe/src/widget/settings/view.rs

use crate::widget::SettingsWidget;
use crate::widget::Widget;

use imgui::Condition;
use imgui::StyleColor;
use imgui::Ui;
use imgui::WindowFlags;

impl<'a> Widget<()> for SettingsWidget<'a> {
    fn draw(&mut self, ui: &Ui, _: &mut ()) {
        if !self.is_open {
            return;
        }

        let display_size = ui.io().display_size;

        let style = ui.push_style_color(StyleColor::WindowBg, [0.1, 0.1, 0.1, 1.0]);
        let child_style = ui.push_style_color(StyleColor::ChildBg, [0.1, 0.1, 0.1, 1.0]);

        let flags = WindowFlags::NO_DOCKING
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_SCROLLBAR
            | WindowFlags::NO_SCROLL_WITH_MOUSE;

        ui.window("Настройки###settings_widget")
            .size(display_size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .flags(flags)
            .build(|| {
                let window_width = ui.window_size()[0];
                let window_height = ui.window_size()[1];

                let button_width = 120.0;
                let button_height = 30.0;

                let total_buttons_width = button_width;

                let bottom_panel_height = button_height + 32.0;
                let scroll_zone_height = window_height - bottom_panel_height;

                ui.child_window("settings_scroll_zone")
                    .size([0.0, scroll_zone_height])
                    .scroll_bar(true)
                    .build(|| {
                        ui.text_disabled("Параметры обновления");
                        ui.separator();
                        ui.text("Адрес репозитория:");
                        ui.input_text(
                            "###url_repository",
                            &mut self.data.application_setting.url_repository,
                        )
                        .build();

                        ui.checkbox(
                            "Проверять обновление при старте mvtool",
                            &mut self.data.application_setting.try_update,
                        );

                        ui.separator();
                    });

                ui.set_cursor_pos([0.0, window_height - bottom_panel_height]);

                ui.child_window("settings_bottom_bar")
                    .size([0.0, bottom_panel_height])
                    .scroll_bar(false)
                    .build(|| {
                        ui.set_cursor_pos([0.0, 0.0]);
                        ui.separator();

                        let btn_x = window_width - total_buttons_width - 16.0;
                        let btn_y = (bottom_panel_height - button_height) / 2.0;

                        ui.set_cursor_pos([btn_x, btn_y]);

                        if ui.button_with_size("Готово", [button_width, button_height]) {
                            ui.close_current_popup();
                            self.is_open = false;
                        }
                    });
            });

        child_style.pop();
        style.pop();
    }
}
