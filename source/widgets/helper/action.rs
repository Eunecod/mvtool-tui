// source/widgets/helper/action.rs

use imgui::Ui;
use imgui::StyleVar;

use std::process::Command;

use super::Executor;
use super::SelectionContext;

pub struct ActionWidget {
    pub label: String,
    pub horizontal: bool,
    pub index: usize,

    pub selection_context: SelectionContext,
}

impl ActionWidget {
    pub fn draw<T: Executor>(&mut self, ui: &Ui, items: &mut Vec<T>) {
        if items.is_empty() {
            return;
        }

        let style = ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]));

        for (i, item) in items.iter_mut().enumerate() {
            /* Создаем виджет исполнителя */
            {
                if i == self.index {
                    ui.text_colored([1.0, 1.0, 0.2, 1.0], &item.name());
                } else {
                    ui.text_colored([0.8, 0.8, 0.8, 1.0], &item.name());
                }

                ui.same_line();

                let button_width = 30.0;
                ui.set_cursor_pos([ui.content_region_max()[0] - button_width, ui.cursor_pos()[1]]);
                if ui.button_with_size(format!("▶##{}_{}", self.label, i), [button_width, 0.0]) {
                    self.execute_command(item.command());
                }

                if ui.is_window_focused() {
                    if i == self.index && ui.is_key_pressed(imgui::Key::Space) {
                        self.execute_command(item.command());
                    }
                }
            }


            if self.horizontal {
                ui.same_line();
            }

            if ui.is_item_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                self.index = i;
            }
        }

        if ui.is_window_focused() {
            if (!self.horizontal && ui.is_key_pressed(imgui::Key::UpArrow)) || (self.horizontal && ui.is_key_pressed(imgui::Key::LeftArrow)) {
                self.index = self.index.saturating_sub(1);
            }
        
            if (!self.horizontal && ui.is_key_pressed(imgui::Key::DownArrow)) || (self.horizontal && ui.is_key_pressed(imgui::Key::RightArrow)) {
                self.index = (self.index + 1).min(items.len() - 1);
            }
        }

        style.pop();
    }

    fn execute_command(&self, command: &str) {
        let command = command.to_string();
        
        std::thread::spawn(move || {
            #[cfg(target_os = "windows")]
            {
                match Command::new("cmd").args(&["/C", "start", "mvtool execute", "cmd", "/K", &command]).current_dir(".").spawn() {
                    Ok(_) => {
                        println!("Выполнен команда: {}", command)
                    }
                    Err(error) => {
                        eprintln!("Ошибка выполнения скрипта: {}", error)
                    }
                }
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                match Command::new("sh").args(&["-c", &command]).spawn() {
                    Ok(_) => {
                        println!("Выполнен команда: {}", command)
                    }
                    Err(error) => {
                        eprintln!("Ошибка выполнения скрипта: {}", error)
                    }
                }
            }
        });
    }

    pub fn try_reset(&mut self, context: SelectionContext) {
        if self.selection_context != context {
            self.index = 0;
            self.selection_context = context;
        }
    }
}