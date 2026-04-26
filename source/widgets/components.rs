// source/widgets/components.rs

use imgui::Ui;
use imgui::Condition;

use super::Widget;
use super::helper::Item;
use super::helper::SelectionContext;
use super::helper::menu::MenuWidget;

use crate::models::Root;
use crate::models::Component;

pub struct ComponentsWidget {
    menu: MenuWidget
}

impl ComponentsWidget {
    pub fn new() -> Self {
        Self {
            menu: MenuWidget {
                label: "##components".to_string(),
                horizontal: false,
                index: 0,

                selection_context: SelectionContext { context: vec![0, 0] }
            }
        }
    }
}

impl Widget<Root> for ComponentsWidget {
    fn draw(&mut self, ui: &Ui, root: &mut Root) {
        ui.window("Компоненты###components_window")
            .size([0.0, 0.0], Condition::Always)
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

                        self.menu.try_reset(SelectionContext { context: vec![project_index, configure_index] });
                        self.menu.draw(ui, &mut configure.components);
                    }
                }
            }
        );
    }
}

impl Item for Component {
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