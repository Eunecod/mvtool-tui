// libs/mvframe/src/widget/plugins_manager/view.rs

use crate::widget::PluginManagerWidget;
use crate::widget::Widget;
use crate::widget::controls::Shortcut;

use imgui::Condition;
use imgui::StyleColor;
use imgui::Ui;
use imgui::WindowFlags;

use mvcore::events::Command;
use mvplugin::core::plugin::PluginState;
use mvplugin::loader::Loader;

impl<'a> Widget<()> for PluginManagerWidget<'a> {
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

        ui.window("Менеджер плагинов###plugin_manager_widget")
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

                ui.child_window("#plugin_manager_scroll_zone")
                    .size([0.0, scroll_zone_height])
                    .scroll_bar(true)
                    .build(|| {
                        for (file_name, plugin) in self.data.plugins.iter_mut() {
                            ui.separator();

                            let metadata = &plugin.metadata;
                            ui.text(format!("Название:         {}", &metadata.name));
                            ui.text(format!("Версия плагина:   {}", &metadata.version));
                            ui.text(format!("Автор плагина:    {}", &metadata.author));
                            ui.text(format!("Описание плагина: {}", &metadata.description));
                            ui.spacing();
                            ui.text(format!("Зависимость:      {}", &metadata.requirement));
                            ui.spacing();

                            if plugin.has_hook("on_start") {
                                let mut shortcut = Shortcut::new(plugin.shortcut.clone());
                                if shortcut.draw(ui, file_name) {
                                    plugin.shortcut = shortcut.key.clone();
                                }
                            }

                            let mut enabled = matches!(plugin.state, PluginState::Active(_));
                            ui.checkbox(
                                &format!("Активировать##item_{}", plugin.metadata.name),
                                &mut enabled,
                            );

                            if !enabled {
                                plugin.state = PluginState::Disabled;
                            } else if matches!(plugin.state, PluginState::Disabled) {
                                match Loader::new(self.data.tx.clone())
                                    .load_instance(format!("./plugins/{}.lua", file_name))
                                {
                                    Ok(instance) => {
                                        plugin.state = PluginState::Active(Box::new(instance));
                                    }
                                    Err(error) => {
                                        let _ =
                                            self.data.tx.blocking_send(Command::ShowMessageBox(
                                                "Ошибка".into(),
                                                format!("Не удалось включить плагин:\n{}", error),
                                                Box::new(|| {}),
                                            ));
                                    }
                                }
                            }

                            ui.separator();
                        }
                    });

                ui.set_cursor_pos([0.0, window_height - bottom_panel_height]);

                ui.child_window("#plugin_manager_bottom_bar")
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
