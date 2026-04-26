// source/widgets/messagebox.rs

use imgui::Ui;

use super::Widget;

use crate::models::Root;

pub struct SimpleMessageBox {
    pub title: String,
    pub message: String,
    pub is_open: bool,
    pub on_yes: Option<Box<dyn FnMut() + Send>>,
    pub on_no: Option<Box<dyn FnMut() + Send>>
}

impl Widget<Root> for SimpleMessageBox {
    fn draw(&mut self, ui: &Ui, _root: &mut Root) {
        if self.is_open {
            ui.open_popup(&self.title);
            self.is_open = false;
        }

        ui.modal_popup_config(&self.title)
            .always_auto_resize(true)
            .build(|| {
                ui.text(&self.message);
                ui.separator();

                if ui.button("Да") {
                    if let Some(callback) = self.on_yes.as_mut() {
                        callback();
                    }

                    ui.close_current_popup();
                }

                ui.same_line();

                if ui.button("Нет") {
                    if let Some(callback) = self.on_no.as_mut() {
                        callback();
                    }

                    ui.close_current_popup();
                }
            });
    }
}
