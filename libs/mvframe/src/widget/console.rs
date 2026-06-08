// libs/mvframe/src/widget/console.rs

use super::Widget;

use imgui::Condition;
use imgui::Ui;

use mvcore::events::Type;

struct LogEntry {
    message: String,
    message_type: Type,
}

pub struct ConsoleWidget {
    entries: Vec<LogEntry>,
    auto_scroll: bool,
}

impl ConsoleWidget {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            auto_scroll: true,
        }
    }

    pub fn add(&mut self, message: &str, message_type: Type) {
        if self.entries.len() > 500 {
            self.entries.remove(0);
        }

        self.entries.push(LogEntry {
            message: message.to_string(),
            message_type,
        });
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn set_auto_scroll(&mut self, enabled: bool) {
        self.auto_scroll = enabled;
    }
}

impl Widget<()> for ConsoleWidget {
    fn draw(&mut self, ui: &Ui, _: &mut ()) {
        ui.window("Консоль###console_window")
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                let button_width = 100.0;

                ui.set_cursor_pos([
                    ui.content_region_max()[0] - button_width,
                    ui.cursor_pos()[1],
                ]);
                if ui.button_with_size("Очистить", [button_width, 0.0]) {
                    self.clear();
                }

                ui.separator();

                ui.child_window("console_scroll_area")
                    .size([0.0, 0.0])
                    .build(|| {
                        for entry in &self.entries {
                            let (color, prefix) = match entry.message_type {
                                Type::Message => ([0.5, 0.7, 1.0, 1.0], "[message]:"),
                                Type::Success => ([0.0, 1.0, 0.0, 1.0], "[success]:"),
                                Type::Warning => ([1.0, 0.8, 0.0, 1.0], "[warning]:"),
                                Type::Error => ([1.0, 0.2, 0.2, 1.0], "[ error ]:"),
                            };

                            ui.text_colored(color, prefix);

                            ui.same_line();

                            ui.text_wrapped(&entry.message);
                        }

                        if self.auto_scroll && ui.scroll_y() >= ui.scroll_max_y() {
                            ui.set_scroll_here_y_with_ratio(1.0);
                        }
                    });
            });
    }
}
