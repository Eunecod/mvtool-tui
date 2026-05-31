// libs/mvframe/src/widget/update.rs

use super::Widget;
use crate::widget::controls::Link;

use tokio::sync::mpsc;

use imgui::Ui;

use mvcore::events::Command;

pub struct UpdatesWidget {
    tx: mpsc::Sender<Command>,
    title: String,
    version: String,
    link: Option<Link>,
    is_open: bool,
}

impl UpdatesWidget {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            tx,
            title: "Доступно обновление".into(),
            version: "".into(),
            link: None,
            is_open: false,
        }
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn set_link(&mut self, link: String) {
        let text = link.clone();
        let url = link.clone();

        self.link = Some(Link::new(text, url));
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }
}

impl Widget<()> for UpdatesWidget {
    fn draw(&mut self, ui: &Ui, _: &mut ()) {
        if self.is_open {
            ui.open_popup(&self.title);
        }

        ui.modal_popup_config(&self.title)
            .always_auto_resize(true)
            .build(|| {
                ui.separator();

                ui.text_colored(
                    [0.9, 0.7, 0.1, 1.0],
                    format!("Доступна новая версия {}. Хотите скачать?", self.version),
                );

                ui.text("");

                if let Some(link) = self.link.as_mut() {
                    ui.text("Информация о релизе:");
                    link.draw(ui);
                } else {
                    ui.text_wrapped("Информации о релизе нет");
                }

                ui.separator();

                let button_width = 120.0;
                let total_buttons_width = (button_width * 2.0) + ui.clone_style().item_spacing[0];

                ui.set_cursor_pos([
                    ui.content_region_max()[0] - total_buttons_width,
                    ui.cursor_pos()[1],
                ]);

                if ui.button_with_size("Обновить", [button_width, 0.0]) {
                    ui.close_current_popup();
                    self.is_open = false;

                    let _ = self.tx.blocking_send(Command::Update());
                }

                ui.same_line();

                if ui.button_with_size("Отказаться", [button_width, 0.0]) {
                    ui.close_current_popup();
                    self.is_open = false;
                }
            });
    }
}
