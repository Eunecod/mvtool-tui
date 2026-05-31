// libs/mvframe/src/widget/configures.rs

use super::Widget;
use super::controls::List;
use super::controls::ListItem;

use imgui::Condition;
use imgui::Ui;

use mvcore::io::Configure;

impl ListItem for Configure {
    fn name(&self) -> &str {
        &self.name
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
}

pub struct ConfiguresWidget;

impl ConfiguresWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Vec<Configure>> for ConfiguresWidget {
    fn draw(&mut self, ui: &Ui, configures: &mut Vec<Configure>) {
        ui.window("Конфигурации###configures_window")
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                let list = List {
                    label: "##configures".to_string(),
                };
                list.draw(ui, configures);

                if let Some(configure) =
                    configures.iter_mut().find(|configures| configures.selected)
                {
                    ui.checkbox("Очистить директорию", &mut configure.clean_destination);
                }
            });
    }
}
