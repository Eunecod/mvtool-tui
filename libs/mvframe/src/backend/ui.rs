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

struct ImguiClipboardBridge(mvcore::adapter::AdapterClipboard);

impl imgui::ClipboardBackend for ImguiClipboardBridge {
    fn get(&mut self) -> Option<String> {
        self.0.get()
    }

    fn set(&mut self, text: &str) {
        self.0.set(text);
    }
}

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
        self.context.set_ini_filename(None);
        self.context
            .set_clipboard_backend(ImguiClipboardBridge(mvcore::adapter::AdapterClipboard));

        let Ok(content) = std::fs::read_to_string("config.ini") else {
            return;
        };
        self.context.load_ini_settings(&content);

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

    pub fn key(&self, key_code: winit::keyboard::KeyCode) -> Option<imgui::Key> {
        match key_code {
            winit::keyboard::KeyCode::KeyA => Some(imgui::Key::A),
            winit::keyboard::KeyCode::KeyB => Some(imgui::Key::B),
            winit::keyboard::KeyCode::KeyC => Some(imgui::Key::C),
            winit::keyboard::KeyCode::KeyD => Some(imgui::Key::D),
            winit::keyboard::KeyCode::KeyE => Some(imgui::Key::E),
            winit::keyboard::KeyCode::KeyF => Some(imgui::Key::F),
            winit::keyboard::KeyCode::KeyG => Some(imgui::Key::G),
            winit::keyboard::KeyCode::KeyH => Some(imgui::Key::H),
            winit::keyboard::KeyCode::KeyI => Some(imgui::Key::I),
            winit::keyboard::KeyCode::KeyJ => Some(imgui::Key::J),
            winit::keyboard::KeyCode::KeyK => Some(imgui::Key::K),
            winit::keyboard::KeyCode::KeyL => Some(imgui::Key::L),
            winit::keyboard::KeyCode::KeyM => Some(imgui::Key::M),
            winit::keyboard::KeyCode::KeyN => Some(imgui::Key::N),
            winit::keyboard::KeyCode::KeyO => Some(imgui::Key::O),
            winit::keyboard::KeyCode::KeyP => Some(imgui::Key::P),
            winit::keyboard::KeyCode::KeyQ => Some(imgui::Key::Q),
            winit::keyboard::KeyCode::KeyR => Some(imgui::Key::R),
            winit::keyboard::KeyCode::KeyS => Some(imgui::Key::S),
            winit::keyboard::KeyCode::KeyT => Some(imgui::Key::T),
            winit::keyboard::KeyCode::KeyU => Some(imgui::Key::U),
            winit::keyboard::KeyCode::KeyV => Some(imgui::Key::V),
            winit::keyboard::KeyCode::KeyW => Some(imgui::Key::W),
            winit::keyboard::KeyCode::KeyX => Some(imgui::Key::X),
            winit::keyboard::KeyCode::KeyY => Some(imgui::Key::Y),
            winit::keyboard::KeyCode::KeyZ => Some(imgui::Key::Z),
            winit::keyboard::KeyCode::Digit0 => Some(imgui::Key::Alpha0),
            winit::keyboard::KeyCode::Digit1 => Some(imgui::Key::Alpha1),
            winit::keyboard::KeyCode::Digit2 => Some(imgui::Key::Alpha2),
            winit::keyboard::KeyCode::Digit3 => Some(imgui::Key::Alpha3),
            winit::keyboard::KeyCode::Digit4 => Some(imgui::Key::Alpha4),
            winit::keyboard::KeyCode::Digit5 => Some(imgui::Key::Alpha5),
            winit::keyboard::KeyCode::Digit6 => Some(imgui::Key::Alpha6),
            winit::keyboard::KeyCode::Digit7 => Some(imgui::Key::Alpha7),
            winit::keyboard::KeyCode::Digit8 => Some(imgui::Key::Alpha8),
            winit::keyboard::KeyCode::Digit9 => Some(imgui::Key::Alpha9),
            winit::keyboard::KeyCode::Escape => Some(imgui::Key::Escape),
            winit::keyboard::KeyCode::Tab => Some(imgui::Key::Tab),
            winit::keyboard::KeyCode::Space => Some(imgui::Key::Space),
            winit::keyboard::KeyCode::Enter => Some(imgui::Key::Enter),
            winit::keyboard::KeyCode::Backspace => Some(imgui::Key::Backspace),
            winit::keyboard::KeyCode::Insert => Some(imgui::Key::Insert),
            winit::keyboard::KeyCode::Delete => Some(imgui::Key::Delete),
            winit::keyboard::KeyCode::ArrowLeft => Some(imgui::Key::LeftArrow),
            winit::keyboard::KeyCode::ArrowRight => Some(imgui::Key::RightArrow),
            winit::keyboard::KeyCode::ArrowUp => Some(imgui::Key::UpArrow),
            winit::keyboard::KeyCode::ArrowDown => Some(imgui::Key::DownArrow),
            winit::keyboard::KeyCode::PageUp => Some(imgui::Key::PageUp),
            winit::keyboard::KeyCode::PageDown => Some(imgui::Key::PageDown),
            winit::keyboard::KeyCode::Home => Some(imgui::Key::Home),
            winit::keyboard::KeyCode::End => Some(imgui::Key::End),
            winit::keyboard::KeyCode::CapsLock => Some(imgui::Key::CapsLock),
            winit::keyboard::KeyCode::ScrollLock => Some(imgui::Key::ScrollLock),
            winit::keyboard::KeyCode::NumLock => Some(imgui::Key::NumLock),
            winit::keyboard::KeyCode::PrintScreen => Some(imgui::Key::PrintScreen),
            winit::keyboard::KeyCode::Pause => Some(imgui::Key::Pause),
            winit::keyboard::KeyCode::Semicolon => Some(imgui::Key::Semicolon),
            winit::keyboard::KeyCode::Equal => Some(imgui::Key::Equal),
            winit::keyboard::KeyCode::Comma => Some(imgui::Key::Comma),
            winit::keyboard::KeyCode::Minus => Some(imgui::Key::Minus),
            winit::keyboard::KeyCode::Period => Some(imgui::Key::Period),
            winit::keyboard::KeyCode::Slash => Some(imgui::Key::Slash),
            winit::keyboard::KeyCode::Backquote => Some(imgui::Key::GraveAccent),
            winit::keyboard::KeyCode::BracketLeft => Some(imgui::Key::LeftBracket),
            winit::keyboard::KeyCode::Backslash => Some(imgui::Key::Backslash),
            winit::keyboard::KeyCode::BracketRight => Some(imgui::Key::RightBracket),
            winit::keyboard::KeyCode::Quote => Some(imgui::Key::Apostrophe),
            winit::keyboard::KeyCode::ShiftLeft => Some(imgui::Key::LeftShift),
            winit::keyboard::KeyCode::ShiftRight => Some(imgui::Key::RightShift),
            winit::keyboard::KeyCode::ControlLeft => Some(imgui::Key::LeftCtrl),
            winit::keyboard::KeyCode::ControlRight => Some(imgui::Key::RightCtrl),
            winit::keyboard::KeyCode::AltLeft => Some(imgui::Key::LeftAlt),
            winit::keyboard::KeyCode::AltRight => Some(imgui::Key::RightAlt),
            winit::keyboard::KeyCode::SuperLeft => Some(imgui::Key::LeftSuper),
            winit::keyboard::KeyCode::SuperRight => Some(imgui::Key::RightSuper),
            winit::keyboard::KeyCode::Numpad0 => Some(imgui::Key::Keypad0),
            winit::keyboard::KeyCode::Numpad1 => Some(imgui::Key::Keypad1),
            winit::keyboard::KeyCode::Numpad2 => Some(imgui::Key::Keypad2),
            winit::keyboard::KeyCode::Numpad3 => Some(imgui::Key::Keypad3),
            winit::keyboard::KeyCode::Numpad4 => Some(imgui::Key::Keypad4),
            winit::keyboard::KeyCode::Numpad5 => Some(imgui::Key::Keypad5),
            winit::keyboard::KeyCode::Numpad6 => Some(imgui::Key::Keypad6),
            winit::keyboard::KeyCode::Numpad7 => Some(imgui::Key::Keypad7),
            winit::keyboard::KeyCode::Numpad8 => Some(imgui::Key::Keypad8),
            winit::keyboard::KeyCode::Numpad9 => Some(imgui::Key::Keypad9),
            winit::keyboard::KeyCode::NumpadDecimal => Some(imgui::Key::KeypadDecimal),
            winit::keyboard::KeyCode::NumpadDivide => Some(imgui::Key::KeypadDivide),
            winit::keyboard::KeyCode::NumpadMultiply => Some(imgui::Key::KeypadMultiply),
            winit::keyboard::KeyCode::NumpadSubtract => Some(imgui::Key::KeypadSubtract),
            winit::keyboard::KeyCode::NumpadAdd => Some(imgui::Key::KeypadAdd),
            winit::keyboard::KeyCode::NumpadEnter => Some(imgui::Key::KeypadEnter),
            winit::keyboard::KeyCode::NumpadEqual => Some(imgui::Key::KeypadEqual),
            winit::keyboard::KeyCode::F1 => Some(imgui::Key::F1),
            winit::keyboard::KeyCode::F2 => Some(imgui::Key::F2),
            winit::keyboard::KeyCode::F3 => Some(imgui::Key::F3),
            winit::keyboard::KeyCode::F4 => Some(imgui::Key::F4),
            winit::keyboard::KeyCode::F5 => Some(imgui::Key::F5),
            winit::keyboard::KeyCode::F6 => Some(imgui::Key::F6),
            winit::keyboard::KeyCode::F7 => Some(imgui::Key::F7),
            winit::keyboard::KeyCode::F8 => Some(imgui::Key::F8),
            winit::keyboard::KeyCode::F9 => Some(imgui::Key::F9),
            winit::keyboard::KeyCode::F10 => Some(imgui::Key::F10),
            winit::keyboard::KeyCode::F11 => Some(imgui::Key::F11),
            winit::keyboard::KeyCode::F12 => Some(imgui::Key::F12),
            _ => None,
        }
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
