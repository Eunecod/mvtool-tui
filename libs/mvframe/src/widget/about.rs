// libs/mvframe/src/widgets/about.rs

use super::Widget;
use crate::widget::controls::Link;

use imgui::Ui;

pub struct AboutWidget {
    title: String,
    qr: String,
    link: Link,

    is_open: bool,
}

impl AboutWidget {
    pub fn new() -> Self {
        Self {
            title: "О программе".into(),
            qr: mvcore::service::qrcode("https://github.com/Eunecod/mvtool-tui"),
            link: Link::new(
                "https://github.com/Eunecod/mvtool-tui".into(),
                "https://github.com/Eunecod/mvtool-tui".into(),
            ),
            is_open: false,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }
}

impl Widget<()> for AboutWidget {
    fn draw(&mut self, ui: &Ui, _: &mut ()) {
        if self.is_open {
            ui.open_popup(&self.title);
        }

        ui.modal_popup_config(&self.title)
			.always_auto_resize(true)
			.build(|| {
				let banner = [
					("                                    888                     888             ", [0.31, 0.63, 1.00, 1.00]),
					("                                    888                     888             ", [0.35, 0.70, 1.00, 1.00]),
					("                                    888                     888             ", [0.45, 0.80, 1.00, 1.00]),
					("             88888b.d88b.  888  888 888888 .d88b.   .d88b.  888             ", [0.20, 0.55, 0.95, 1.00]),
					("             888 '888 '88b 888  888 888   d88''88b d88''88b 888             ", [0.18, 0.50, 0.90, 1.00]),
					("             888  888  888 Y88  88P 888   888  888 888  888 888             ", [0.15, 0.45, 0.85, 1.00]),
					("             888  888  888  Y8bd8P  Y88b. Y88..88P Y88..88P 888             ", [0.12, 0.40, 0.80, 1.00]),
					("             888  888  888   Y88P    'Y888 'Y88P'   'Y88P'  888             ", [0.10, 0.35, 0.75, 1.00]),
					("────────────────────────────────────────────────────────────────────────────", [0.40, 0.40, 0.40, 1.00]),
				];

				let window_width = ui.window_size()[0];

				for (text, color) in banner {
					let text_width = ui.calc_text_size(text)[0];

					let x = (window_width - text_width) * 0.5;

					let pos = ui.cursor_pos();
					ui.set_cursor_pos([x.max(0.0), pos[1]]);

					ui.text_colored(color, text);
				}

				ui.group(|| {
					ui.text(&self.qr);
					self.link.draw(ui);
				});

				ui.same_line();

				let yellow = [1.00, 0.84, 0.00, 1.00];
				let gray = [0.55, 0.55, 0.55, 1.00];
				let blue = [0.31, 0.63, 1.00, 1.00];

				ui.group(|| {
					ui.text_colored(yellow, "[esud]");
					ui.same_line();
					ui.text(format!("mvtool v{}", mvcore::service::version()));

					ui.text_colored(gray, "30/01/2026");

					ui.text("");

					ui.text("Инструмент      на      базе      Rust");
					ui.text("для  автоматизации  рабочего  процесса");
					ui.text("и выполнения вспомогательных сценариев");

					ui.text("");
					ui.text("");

					ui.text("Лицензия: MIT / Apache 2.0");

					ui.text("");

					ui.text_colored(blue, "platforms:");
					ui.same_line();

					let platform = format!(
						"{}_{} {}", std::env::consts::OS, std::env::consts::ARCH, mvcore::service::bit_depth()
					);
					ui.text_colored(gray, platform);

					ui.text_colored(blue, "user:");
					ui.same_line();

					let user = std::env::var("USERNAME").or_else(|_| std::env::var("USER")).unwrap_or_else(|_| "unknown".into());
					ui.text_colored(gray, format!("     @{}", user));
				});

				ui.separator();

				let button_width = 100.0;

                ui.set_cursor_pos([
                    ui.content_region_max()[0] - button_width,
                    ui.cursor_pos()[1],
                ]);

                if ui.button_with_size("Принято", [button_width, 0.0]) {
                    ui.close_current_popup();
                    self.is_open = false;
                }
			});
    }
}
