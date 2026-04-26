// source/widgets/projects.rs

use imgui::Ui;
use imgui::Condition;

use super::Widget;
use super::helper::Item;
use super::helper::list::ListWidget;

use crate::models::Root;
use crate::models::Project;

pub struct ProjectsWidget;

impl ProjectsWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Root> for ProjectsWidget {
    fn draw(&mut self, ui: &Ui, root: &mut Root) {
        ui.window("Проекты###projects_window")
            .size([0.0, 0.0], Condition::Always)
            .build(|| {
                ui.separator();

                let list = ListWidget {
                    label: "##projects".to_string(),
                };

                list.draw(ui, &mut root.projects);
            }
        );
    }
}

impl Item for Project {
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