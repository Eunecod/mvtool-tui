// libs/mvframe/src/widget/components.rs

use super::Widget;
use super::controls::Menu;
use super::controls::MenuItem;

use imgui::Condition;
use imgui::Ui;

use mvcore::io::Component;

impl MenuItem for Component {
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

pub struct ComponentsWidget {
    menu: Menu,
}

impl ComponentsWidget {
    pub fn new() -> Self {
        Self {
            menu: Menu {
                label: "##components".to_string(),
                horizontal: false,
                index: 0,
            },
        }
    }
}

impl Widget<Vec<Component>> for ComponentsWidget {
    fn draw(&mut self, ui: &Ui, components: &mut Vec<Component>) {
        ui.window("Компоненты###components_window")
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                self.menu.draw(ui, components);
            });
    }
}
