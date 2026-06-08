// libs/mvframe/src/widget/tabmenu.rs

use super::Widget;

use imgui::Ui;

use tokio::sync::mpsc;

use mvcore::events::Command;

pub struct TabmenuWidget {
    tx: mpsc::Sender<Command>,
}

impl TabmenuWidget {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self { tx }
    }
}

impl Widget<()> for TabmenuWidget {
    fn draw(&mut self, ui: &Ui, _: &mut ()) {
        ui.main_menu_bar(|| {
            ui.menu("Файл", || {
                if ui.menu_item("Менеджер плагинов") {
                    let _ = self.tx.blocking_send(Command::PluginManager());
                }
                if ui.menu_item("Настройки") {}

                ui.separator();

                if ui.menu_item("Выход") {
                    let _ = self.tx.blocking_send(Command::Exit());
                }
            });

            ui.menu("О программе", || {
                if ui.menu_item("Что это?!") {
                    let _ = self.tx.blocking_send(Command::About());
                }
            });
        });
    }
}
