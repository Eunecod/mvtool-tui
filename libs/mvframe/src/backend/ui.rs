// libs/mvframe/src/backend/ui.rs

use imgui::ConfigFlags;
use imgui::Context;
use imgui::Direction;
use imgui::DrawData;
use imgui::FontConfig;
use imgui::FontGlyphRanges;
use imgui::FontSource;
use imgui::StyleColor;
use imgui::Ui;

use imgui_winit_support::HiDpiMode;
use imgui_winit_support::WinitPlatform;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::window::Window;

use std::path::PathBuf;

pub struct UiState {
    platform: WinitPlatform,
    context: Context,
}

impl UiState {
    pub fn new() -> Self {
        let mut context = Context::create();
        Self {
            platform: WinitPlatform::new(&mut context),
            context,
        }
    }

    pub fn setup(&mut self) {
        self.context.io_mut().config_flags |= ConfigFlags::DOCKING_ENABLE;
        self.context
            .set_ini_filename(Some(PathBuf::from("config.ini")));

        self.setup_style();
    }

    pub fn setup_fonts(&mut self, font_data: &'static [u8]) {
        self.context.fonts().add_font(&[FontSource::TtfData {
            data: font_data,
            size_pixels: 20.0,
            config: Some(FontConfig {
                glyph_ranges: FontGlyphRanges::from_slice(&[0x0020, 0xFFFF, 0]),
                pixel_snap_h: true,
                ..FontConfig::default()
            }),
        }]);
    }

    pub fn attach_window(&mut self, window: &Window) {
        self.platform
            .attach_window(self.context.io_mut(), window, HiDpiMode::Default);
    }

    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn build<FBuilder>(&mut self, window: &Window, callback: FBuilder)
    where
        FBuilder: FnOnce(&mut Ui),
    {
        let _ = self.platform.prepare_frame(self.context.io_mut(), window);

        let ui = self.context.frame();
        callback(ui);

        let _ = self.platform.prepare_render(ui, window);
    }

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) {
        let wrapper_event: Event<()> = Event::WindowEvent {
            window_id: window.id(),
            event: event.clone(),
        };

        self.platform
            .handle_event(self.context.io_mut(), window, &wrapper_event);
    }

    pub fn data(&mut self) -> &DrawData {
        self.context.render()
    }

    fn setup_style(&mut self) {
        let style = self.context.style_mut();

        style.window_padding = [20.0, 15.0];
        style.frame_padding = [10.0, 03.0];
        style.item_spacing = [10.0, 10.0];
        style.item_inner_spacing = [10.0, 06.0];
        style.colors[StyleColor::TabUnfocusedActive as usize] = [0.0, 0.0, 0.0, 0.0];
        style.colors[StyleColor::TabActive as usize] = [0.0, 0.0, 0.0, 0.0];
        style.colors[StyleColor::TabHovered as usize] = [0.0, 0.0, 0.0, 0.0];
        style.window_menu_button_position = Direction::None;

        style.anti_aliased_lines = false;
        style.anti_aliased_fill = false;
    }
}
