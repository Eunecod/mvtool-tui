// libs/mvframe/src/widget/scripts.rs

use super::Widget;
use super::controls::Action;
use super::controls::Executor;

use tokio::sync::mpsc;

use imgui::Condition;
use imgui::Ui;

use mvcore::events::Command;
use mvcore::events::Type;
use mvcore::io::Script;

impl Executor for Script {
    fn name(&self) -> &str {
        &self.name
    }

    fn command(&self) -> &str {
        &self.command
    }
}

pub struct ScriptsWidget {
    tx: mpsc::Sender<Command>,
    action: Action,
}

impl ScriptsWidget {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            tx,
            action: Action {
                label: "##scripts".to_string(),
                horizontal: false,
                index: 0,
            },
        }
    }
}

impl Widget<Vec<Script>> for ScriptsWidget {
    fn draw(&mut self, ui: &Ui, scripts: &mut Vec<Script>) {
        ui.window("Скрипты###scripts_window")
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                self.action.draw(ui, scripts, |item| {
                    let tx = self.tx.clone();
                    let task_name = item.name().to_string();

                    let _ = self.tx.blocking_send(Command::Execute(
                        item.name().to_string(),
                        item.command().to_string(),
                        Some(Box::new(move |success| {
                            if success {
                                let _ = tx.blocking_send(Command::Devent(
                                    format!("Задача '{}' успешно выполнена", task_name),
                                    Type::Success,
                                ));
                            } else {
                                let _ = tx.blocking_send(Command::Devent(
                                    format!("Задача '{}' завершилась с ошибкой", task_name),
                                    Type::Error,
                                ));
                            }
                        })),
                    ));
                });
            });
    }
}
