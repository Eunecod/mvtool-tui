// source/application/backend/_sdl2.rs

use sdl2::EventPump;
use sdl2::VideoSubsystem;
use sdl2::EventSubsystem;
use sdl2::video::Window;
use sdl2::video::GLProfile;
use sdl2::video::GLContext;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

const ICON_DATA: &[u8] = include_bytes!("windows/icon.ico");

pub struct SDL2 {
	video: VideoSubsystem,
	event: EventSubsystem,
	window: Window,
	event_pump: EventPump,

	_gl_context: GLContext,
}

impl SDL2 {
	pub fn new() -> Result<Self, String> {
		let sdl = sdl2::init()?;
		let video = sdl.video()?;

		let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

		let mut window = video.window("mvtool", 1280, 720).opengl().resizable().build().map_err(|error| error.to_string())?;
		window.show();
		if let Ok(surface) = Self::load_icon() {
            window.set_icon(surface);
        }

		let gl_context = window.gl_create_context()?;
		window.gl_make_current(&gl_context)?;

		match video.gl_set_swap_interval(1) {
            Ok(_) => {
                println!("Вертикальная синхронизация включена");
            }
            Err(error) => {
				println!("{}", format!("Не удалось включить вертикальную синхронизацию: {}", error));
            }
        }

		Ok(Self {
			video: video,
			event: sdl.event()?,
			window: window,
			event_pump: sdl.event_pump()?,

			_gl_context: gl_context
		})
	}

	pub fn video(&self) -> &VideoSubsystem {
		&self.video
	}

	pub fn event(&self) -> &EventSubsystem {
		&self.event
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn event_pump(&self) -> &EventPump {
		&self.event_pump
	}

	pub fn poll_events<F: FnMut(Event)>(&mut self, mut f: F) {
		for event in self.event_pump.poll_iter() {
            f(event);
        }
	}

	pub fn swap(&self) {
        self.window.gl_swap_window();
    }

	fn load_icon() -> Result<Surface<'static>, String> {
        let img = image::load_from_memory(ICON_DATA).map_err(|error| format!("Ошибка декодирования *.ico: {}", error))?;

		let rgba = img.to_rgba8();
		let (width, height) = rgba.dimensions();

		let boxed: Box<[u8]> = rgba.into_vec().into_boxed_slice();
		let data: &'static mut [u8] = Box::leak(boxed);

		let surface = Surface::from_data(
		    data,
		    width,
		    height,
		    width * 4,
		    PixelFormatEnum::RGBA32,
		).map_err(|error| format!("Ошибка создания поверхности: {}", error))?;

		Ok(surface)
	}
}