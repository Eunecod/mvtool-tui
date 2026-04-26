// source/widgets/configures.rs

use imgui::Ui;
use imgui::Condition;

use super::Widget;
use super::helper::Item;
use super::helper::list::ListWidget;

use crate::models::Root;
use crate::models::Configure;

pub struct ConfiguresWidget;

impl ConfiguresWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Root> for ConfiguresWidget {
    fn draw(&mut self, ui: &Ui, root: &mut Root) {
        ui.window("Конфигурации###configures_window")
            .size([100.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                let list = ListWidget {
                    label: "##configures".to_string(),
                };

                for project in &mut root.projects
                {
                    if !project.selected
                    {
                        continue;
                    }

                    list.draw(ui, &mut project.configures);
                }
            }
        );
    }
}

impl Item for Configure {
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