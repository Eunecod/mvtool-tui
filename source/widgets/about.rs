// source/widgets/about.rs

use imgui::Ui;

use super::Widget;

use crate::service::bit_depth;
use crate::service::qrcode;

use crate::models::Root;

pub struct AboutWidget {
	pub title: String,
	pub is_open: bool
}

impl Widget<Root> for AboutWidget {
	fn draw(&mut self, ui: &Ui, _root: &mut Root) {
		if self.is_open {
			ui.open_popup(&self.title);
			self.is_open = false;
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

				let qr = qrcode("https://github.com/Eunecod/mvtool-tui");

				ui.group(|| {
					ui.text(&qr);
					ui.text("https://github.com/Eunecod/mvtool-tui");
				});

				ui.same_line();

				let yellow = [1.00, 0.84, 0.00, 1.00];
				let gray   = [0.55, 0.55, 0.55, 1.00];
				let blue   = [0.31, 0.63, 1.00, 1.00];
				
				ui.group(|| {
					ui.text_colored(yellow, "[esud]");
					ui.same_line();
					ui.text(format!("mvtool v{}", env!("CARGO_PKG_VERSION")));

					ui.text_colored(gray, "30/01/2026");
					
					ui.text("");

					ui.text("Инструмент      на      базе      Rust");
					ui.text("для  автоматизации  рабочего  процесса");
					ui.text("и выполнения вспомогательных сценариев");

					ui.text("");
					ui.text("");
					ui.text("");

					ui.text("Лицензия: MIT / Apache 2.0");

					ui.text("");

					ui.text_colored(blue, "platforms:");
					ui.same_line();
				
					let platform = format!(
						"{}_{} {}", std::env::consts::OS, std::env::consts::ARCH, bit_depth()
					);
					ui.text_colored(gray, platform);

					ui.text_colored(blue, "user:");
					ui.same_line();
				
					let user = std::env::var("USERNAME").or_else(|_| std::env::var("USER")).unwrap_or_else(|_| "unknown".into());
					ui.text_colored(gray, format!("     @{}", user));
				});

				ui.separator();

				if ui.button("Принято") {
					ui.close_current_popup();
				}
			});
	}
}
