// mvframe/src/widget/controls/list.rs

use imgui::Key;
use imgui::Ui;

pub trait ListItem {
    fn name(&self) -> &str;
    fn selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
}

pub struct List {
    pub label: String,
}

impl List {
    pub fn draw<T>(&self, ui: &Ui, items: &mut Vec<T>)
    where
        T: ListItem,
    {
        if items.is_empty() {
            return;
        }

        let mut selected_index = items.iter().position(|x| x.selected()).unwrap_or(0);
        let current_name = items[selected_index].name();

        ui.set_next_item_width(-1.0);
        if let Some(_) = ui.begin_combo(&self.label, current_name) {
            for (i, item) in items.iter_mut().enumerate() {
                let is_selected = i == selected_index;

                if ui
                    .selectable_config(item.name())
                    .selected(is_selected)
                    .build()
                {
                    selected_index = i;
                }
            }
        }

        if ui.is_window_focused() {
            if (ui.is_key_pressed(Key::UpArrow)) || (ui.is_key_pressed(Key::DownArrow)) {
                selected_index = selected_index.checked_sub(1).unwrap_or(items.len() - 1);
            }
        }

        if ui.is_item_hovered() {
            let wheel = ui.io().mouse_wheel;
            if wheel > 0.0 {
                selected_index = selected_index.checked_sub(1).unwrap_or(items.len() - 1);
            } else if wheel < 0.0 {
                selected_index = (selected_index + 1) % items.len();
            }
        }

        for (i, item) in items.iter_mut().enumerate() {
            item.set_selected(i == selected_index);
        }
    }
}
