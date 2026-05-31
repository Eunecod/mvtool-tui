// libs/mvframe/src/widget/controls/menu.rs

use imgui::Key;
use imgui::MouseButton;
use imgui::StyleVar;
use imgui::Ui;

pub trait MenuItem {
    fn name(&self) -> &str;
    fn selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
}

pub struct Menu {
    pub label: String,
    pub horizontal: bool,
    pub index: usize,
}

impl Menu {
    pub fn draw<T>(&mut self, ui: &Ui, items: &mut Vec<T>)
    where
        T: MenuItem,
    {
        if items.is_empty() {
            return;
        }

        let style = ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]));

        for (i, item) in items.iter_mut().enumerate() {
            let mut selected = item.selected();

            /* Создание чекбокса, с сохраняем его функциональности */
            {
                if ui.checkbox(
                    &format!("##item_{}{}", self.label, item.name()),
                    &mut selected,
                ) {
                    item.set_selected(selected);
                    self.index = i;
                }

                ui.same_line();

                if i == self.index {
                    ui.text_colored([1.0, 1.0, 1.0, 1.0], item.name());
                } else {
                    ui.text_colored([0.55, 0.55, 0.55, 1.0], item.name());
                }
            }

            if self.horizontal {
                ui.same_line();
            }

            if ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Left) {
                item.set_selected(!selected);
                self.index = i;
            }
        }

        if ui.is_window_focused() {
            if (!self.horizontal && ui.is_key_pressed(Key::UpArrow))
                || (self.horizontal && ui.is_key_pressed(Key::LeftArrow))
            {
                self.index = self.index.saturating_sub(1);
            }

            if (!self.horizontal && ui.is_key_pressed(Key::DownArrow))
                || (self.horizontal && ui.is_key_pressed(Key::RightArrow))
            {
                self.index = (self.index + 1).min(items.len() - 1);
            }

            if ui.io().key_ctrl && ui.is_key_pressed(Key::A) {
                let should_select = items.iter().any(|component| !component.selected());
                for item in items.iter_mut() {
                    item.set_selected(should_select);
                }
            }

            if ui.is_key_pressed(Key::Space) {
                let mut value = items[self.index].selected();
                value = !value;

                items[self.index].set_selected(value);
            }
        }

        style.pop();
    }
}
