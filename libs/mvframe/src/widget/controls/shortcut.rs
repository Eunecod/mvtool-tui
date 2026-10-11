// libs/mvframe/src/widget/controls/shortcut.rs

use imgui::{Key, Ui};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Shortcut {
    pub key: Option<u32>,
}

impl Shortcut {
    pub fn new(key: Option<u32>) -> Self {
        Self { key }
    }

    pub fn draw(&mut self, ui: &Ui, label: &str) -> bool {
        let mut changed = false;
        let modal_id = format!("Настройка клавиши##modal_{}", label);

        let shortcut = self
            .key
            .and_then(|code| Key::VARIANTS.iter().find(|&&key| key as u32 == code))
            .map(|&key| format!("{key:?}"))
            .unwrap_or_else(|| "*".to_string());

        if ui.button(format!("{}##btn_{}", shortcut, label)) {
            ui.open_popup(&modal_id);
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Изменить горячую клавишу");
        }

        ui.same_line();
        ui.align_text_to_frame_padding();

        ui.modal_popup_config(&modal_id)
            .always_auto_resize(true)
            .build(|| {
                if ui.is_key_pressed(Key::Escape) {
                    self.key = None;
                    changed = true;

                    ui.close_current_popup();
                } else {
                    for key in Key::VARIANTS {
                        if ui.is_key_pressed(key) {
                            self.key = Some(key as u32);
                            changed = true;

                            ui.close_current_popup();

                            break;
                        }
                    }
                }

                ui.text("Нажмите клавишу на клавиатуре...");
                ui.text_colored([0.6, 0.6, 0.6, 1.0], "(Нажмите ESC для очистки)");
                ui.spacing();
                ui.separator();

                if ui.button("Отмена") {
                    ui.close_current_popup();
                }
            });

        changed
    }
}
