// source/widgets/helper/list.rs

use imgui::Ui;

use super::Item;

pub struct ListWidget {
    pub label: String,
}

impl ListWidget {
    pub fn draw<T: Item>(&self, ui: &Ui, items: &mut Vec<T>) {
        if items.is_empty() {
            return;
        }

        let mut selected_index = items.iter().position(|x| x.selected()).unwrap_or(0);
        let current_name = items[selected_index].name();

        ui.set_next_item_width(-1.0);
        if let Some(_combo) = ui.begin_combo(&self.label, current_name) {
            for (i, item) in items.iter_mut().enumerate() {
                let is_selected = i == selected_index;

                if ui.selectable_config(item.name()).selected(is_selected).build() {
                    selected_index = i;
                }
            }
        }

        if ui.is_window_focused() {
            if (ui.is_key_pressed(imgui::Key::UpArrow)) || (ui.is_key_pressed(imgui::Key::DownArrow))
            {
                selected_index = selected_index.checked_sub(1).unwrap_or(items.len() - 1);
            }
        }

        for (i, item) in items.iter_mut().enumerate() {
            item.set_selected(i == selected_index);
        }
    }
}