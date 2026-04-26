// source/widgets/messagelog.rs

use imgui::Ui;
use super::Widget;
use super::helper::waiter::WaiterWidget;
use super::helper::waiter::WaiterState;

use imgui::Condition;

use crate::models::Root;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MessageType {
    #[default]
    Warning,
    Success,
    Error,
    Info,
}

pub struct LogEntry {
    message: String,
    message_type: MessageType,
}

pub struct ConsoleWidget {
    entries: Vec<LogEntry>,
    auto_scroll: bool,

    waiter_state: WaiterState
}

impl ConsoleWidget {
    pub fn new() -> Self {
        Self { entries: Vec::new(), auto_scroll: true, waiter_state: WaiterState { tick_count: 0, process: false } }
    }
    
    pub fn add(&mut self, message: &str, message_type: MessageType) {
        if self.entries.len() > 500 {
            self.entries.remove(0);
        }
        
        self.entries.push(LogEntry { message: message.to_string(), message_type: message_type });
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    pub fn set_auto_scroll(&mut self, enabled: bool) {
        self.auto_scroll = enabled;
    }

    pub fn pump_waiter(&mut self, state: &mut WaiterState) {
        WaiterWidget::tick(state);
        self.waiter_state = *state;
    }
}

impl Widget<Root> for ConsoleWidget {
    fn draw(&mut self, ui: &Ui, _root: &mut Root) {
        let header: String = format!("Консоль {}###console_window", WaiterWidget::get_frame(&mut self.waiter_state));
        ui.window(&header)
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                let button_width = 100.0;
            
                ui.set_cursor_pos([ui.content_region_max()[0] - button_width, ui.cursor_pos()[1]]);
                if ui.button_with_size("Очистить", [button_width, 0.0]) {
                    self.clear();
                }
            
                ui.separator();
            
                ui.child_window("console_scroll_area")
                    .size([0.0, 0.0])
                    .build(|| {
                        for entry in &self.entries {
                            let (color, prefix) = match entry.message_type {
                                MessageType::Warning => ([1.0, 0.8, 0.0, 1.0], "[warning]:"),
                                MessageType::Success => ([0.0, 1.0, 0.0, 1.0], "[success]:"),
                                MessageType::Error   => ([1.0, 0.2, 0.2, 1.0], "[error]:  "),
                                MessageType::Info    => ([0.5, 0.7, 1.0, 1.0], "[info]:   "),
                            };
            
                            ui.text_colored(color, prefix);

                            ui.same_line();
                            ui.text(&entry.message);
                        }
            
                        if self.auto_scroll && ui.scroll_y() >= ui.scroll_max_y() {
                            ui.set_scroll_here_y_with_ratio(1.0);
                        }
                    }
                );
            }
        );
    }
}