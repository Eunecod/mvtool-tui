// source/widgets/scripts.rs

use imgui::Ui;
use imgui::Condition;

use super::Widget;
use super::helper::Executor;
use super::helper::SelectionContext;
use super::helper::action::ActionWidget;

use crate::models::Root;
use crate::models::Script;

pub struct ScriptsWidget {
    action: ActionWidget
}

impl ScriptsWidget {
    pub fn new() -> Self {
        Self {
            action: ActionWidget {
                label: "##scripts".to_string(),
                horizontal: false,
                index: 0,

                selection_context: SelectionContext { context: vec![0, 0] }
            }
        }
    }
}

impl Widget<Root> for ScriptsWidget {
    fn draw(&mut self, ui: &Ui, root: &mut Root) {
        ui.window("Скрипты###scripts_window")
            .size([0.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                for (project_index, project) in &mut root.projects.iter_mut().enumerate() {
                    if !project.selected {
                        continue;
                    }

                    for (configure_index, configure) in &mut project.configures.iter_mut().enumerate() {
                        if !configure.selected {
                            continue;
                        }

                        self.action.try_reset(SelectionContext { context: vec![project_index, configure_index] });
                        self.action.draw(ui, &mut configure.scripts);
                    }
                }
            }
        );
    }
}

impl Executor for Script {
    fn name(&self) -> &str {
        &self.name
    }

    fn command(&self) -> &str {
        &self.command
    }
}