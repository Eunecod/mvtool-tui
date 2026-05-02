// source/application/backend/_imgui.rs

use imgui::Ui;
use imgui::Context;
use imgui::ConfigFlags;
use imgui::FontSource;
use imgui::FontConfig;
use imgui::FontGlyphRanges;
use imgui::Style;
use imgui::Direction;
use imgui::StyleColor;
use imgui::DrawData;

use imgui_sdl2_support::SdlPlatform;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::EventPump;

use std::path::PathBuf;

use crate::models::Root;

pub struct ImGUI {
	context: Context,
	platform: SdlPlatform,
}

impl ImGUI {
	pub fn new() -> Self {
		let mut context = Context::create();
		let platform = SdlPlatform::new(&mut context);

		Self { context: context, platform: platform }
	}

	pub fn setup_ui(&mut self) {
		self.context.io_mut().config_flags = ConfigFlags::DOCKING_ENABLE;

		self.context.set_ini_filename(Some(PathBuf::from("config.ini")));

		self.context.fonts().add_font(&[
			FontSource::TtfData
			{
			    data: include_bytes!("font/mvtool-bold.ttf"),
			    size_pixels: 20.0,
			    config: Some(FontConfig
			    {
			        glyph_ranges: FontGlyphRanges::from_slice(&[0x0020, 0xFFFF, 0]),
			        ..FontConfig::default()
			    }),
			}
		]);

		self.setup_style();
	}

	fn setup_style(&mut self) {
		let style: &mut Style = self.context.style_mut();

		style.window_padding								  = [20.0, 15.0];
		style.frame_padding									  = [10.0, 03.0];
		style.item_spacing									  = [10.0, 10.0];
		style.item_inner_spacing							  = [10.0, 06.0];
		style.colors[StyleColor::TabUnfocusedActive as usize] = [0.0, 0.0, 0.0, 0.0];
		style.colors[StyleColor::TabActive as usize]		  = [0.0, 0.0, 0.0, 0.0];
		style.colors[StyleColor::TabHovered as usize]		  = [0.0, 0.0, 0.0, 0.0];
		style.window_menu_button_position					  = Direction::None;

		style.anti_aliased_lines                              = false;
		style.anti_aliased_fill                               = false;
	}

	pub fn context(&mut self) -> &mut Context {
		&mut self.context
	}

	pub fn handle_event(&mut self, event: &Event) {
		self.platform.handle_event(&mut self.context, event);
	}

	pub fn prepare_frame(&mut self, window: &Window, event_pump: &EventPump) {
		self.platform.prepare_frame(&mut self.context, window, event_pump)
	}

	pub fn data(&mut self) -> &DrawData {
        self.context.render()
    }

	pub fn paint<F: FnMut(&mut Ui, &mut Root)>(&mut self, mut f: F, root: &mut Root) {
		let ui = self.context.frame();
		f(ui, root);
	}
}
