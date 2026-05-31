// libs/mvframe/src/widget/projects.rs

use super::Widget;
use super::controls::List;
use super::controls::ListItem;

use imgui::Condition;
use imgui::Ui;

use mvcore::io::Project;

impl ListItem for Project {
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

pub struct ProjectsWidget;

impl ProjectsWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Vec<Project>> for ProjectsWidget {
    fn draw(&mut self, ui: &Ui, projects: &mut Vec<Project>) {
        ui.window("Проекты###projects_window")
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                let list = List {
                    label: "##projects".to_string(),
                };
                list.draw(ui, projects);
            });
    }
}
