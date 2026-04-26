// source/widgets/helper/menu.rs

use imgui::Ui;
use imgui::StyleVar;

use super::Item;
use super::SelectionContext;

pub struct MenuWidget {
    pub label: String,
    pub horizontal: bool,
    pub index: usize,

    pub selection_context: SelectionContext,
}

impl MenuWidget {
    pub fn draw<T: Item>(&mut self, ui: &Ui, items: &mut Vec<T>) {
        if items.is_empty() {
            return;
        }

        let style = ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]));

        for (i, item) in items.iter_mut().enumerate() {
            let mut selected = item.selected();

            /* Создание чекбокса, с сохраняем его функциональности */
            {
                if ui.checkbox(&format!("##item_{}{}", self.label, item.name() ), &mut selected) {
                    item.set_selected(selected);
                    self.index = i;
                }

                ui.same_line();

                if i == self.index {
                    ui.text_colored([1.0, 1.0, 1.0, 1.0], item.name());
                }
                else {
                    ui.text_colored([0.55, 0.55, 0.55, 1.0], item.name());
                }
            }

            if self.horizontal {
                ui.same_line();
            }

            if ui.is_item_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                item.set_selected(!selected);
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

            if ui.io().key_ctrl && ui.is_key_pressed(imgui::Key::A) {
                let should_select = items.iter().any(|component| !component.selected());
                for item in items.iter_mut() {
                    item.set_selected(should_select);
                }
            }

            if ui.is_key_pressed(imgui::Key::Space) {
                let mut value = items[self.index].selected();
                value = !value;
        
                items[self.index].set_selected(value);
            }
        }
    
        style.pop();
    }

    pub fn try_reset(&mut self, context: SelectionContext) {
        if self.selection_context != context {
            self.index = 0;
            self.selection_context = context;
        }
    }
}