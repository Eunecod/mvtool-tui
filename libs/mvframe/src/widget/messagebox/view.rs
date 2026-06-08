// libs/mvframe/src/widget/messagebox/view.rs

use crate::widget::MessageBox;
use crate::widget::Widget;

use imgui::Ui;

impl Widget<()> for MessageBox {
    fn draw(&mut self, ui: &Ui, _: &mut ()) {
        let (title, message) = match &mut self.data {
            Some(data) => (&data.title, &data.message),
            None => return,
        };

        if self.is_open {
            ui.open_popup(title);
        }

        let mut result = 0;

        ui.modal_popup_config(title)
            .always_auto_resize(true)
            .build(|| {
                ui.separator();

                ui.text(message);

                ui.separator();

                let button_width = 100.0;
                let total_buttons_width = (button_width * 2.0) + ui.clone_style().item_spacing[0];

                ui.set_cursor_pos([
                    ui.content_region_max()[0] - total_buttons_width,
                    ui.cursor_pos()[1],
                ]);

                if ui.button_with_size("Ок", [button_width, 0.0]) {
                    ui.close_current_popup();
                    result = 1;
                }

                ui.same_line();

                if ui.button_with_size("Отмена", [button_width, 0.0]) {
                    ui.close_current_popup();
                    result = 2;
                }
            });

        match result {
            1 => self.click_ok(),
            2 => self.click_cancel(),
            _ => {}
        }
    }
}
