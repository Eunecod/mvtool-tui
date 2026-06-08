// libs/mvframe/src/widget/controls/link.rs

use imgui::MouseButton;
use imgui::Ui;

pub struct Link {
    pub text: String,
    pub url: String,
    pub is_hovered: bool,
}

impl Link {
    pub fn new(text: String, url: String) -> Self {
        Self {
            text,
            url,
            is_hovered: false,
        }
    }

    pub fn draw(&mut self, ui: &Ui) {
        let color = if self.is_hovered {
            [0.45, 0.75, 1.00, 1.00]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        ui.text_colored(color, &self.text);

        self.is_hovered = ui.is_item_hovered();

        if ui.is_item_clicked_with_button(MouseButton::Left) {
            mvcore::service::open_url(&self.url);
        }
    }
}
