// libs/mvframe/src/widget/controls/action.rs

use imgui::Key;
use imgui::MouseButton;
use imgui::StyleVar;
use imgui::Ui;

pub trait Executor {
    fn name(&self) -> &str;
    fn command(&self) -> &str;
}

pub struct Action {
    pub label: String,
    pub horizontal: bool,
    pub index: usize,
}

impl Action {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            horizontal: false,
            index: 0,
        }
    }

    pub fn draw<T, F>(&mut self, ui: &Ui, items: &mut Vec<T>, mut on_execute: F)
    where
        T: Executor,
        F: FnMut(&T),
    {
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

                ui.set_cursor_pos([
                    ui.content_region_max()[0] - button_width,
                    ui.cursor_pos()[1],
                ]);

                if ui.button_with_size(format!("▶##{}_{}", self.label, i), [button_width, 0.0]) {
                    on_execute(item);
                }

                if ui.is_window_focused() {
                    if i == self.index && ui.is_key_pressed_no_repeat(Key::Space) {
                        on_execute(item);
                    }
                }
            }

            if self.horizontal {
                ui.same_line();
            }

            if ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Left) {
                self.index = i;
            }
        }

        if ui.is_window_focused() {
            if (!self.horizontal && ui.is_key_pressed_no_repeat(Key::UpArrow))
                || (self.horizontal && ui.is_key_pressed_no_repeat(Key::LeftArrow))
            {
                self.index = self.index.saturating_sub(1);
            }

            if (!self.horizontal && ui.is_key_pressed_no_repeat(Key::DownArrow))
                || (self.horizontal && ui.is_key_pressed_no_repeat(Key::RightArrow))
            {
                self.index = (self.index + 1).min(items.len() - 1);
            }
        }

        style.pop();
    }
}
