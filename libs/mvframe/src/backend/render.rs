// libs/mvframe/src/backend/render.rs

use imgui::Context;
use imgui::DrawData;

use std::num::NonZeroU32;

use imgui_glow_renderer::glow;
use imgui_glow_renderer::glow::HasContext;

use imgui_glow_renderer::Renderer;
use imgui_glow_renderer::SimpleTextureMap;

use glutin::config::ConfigTemplateBuilder;
use glutin::config::GlConfig;

use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
use glutin::context::NotCurrentGlContext;
use glutin::context::PossiblyCurrentContext;

use glutin::display::GlDisplay;
use glutin::surface::GlSurface;
use glutin::surface::Surface;
use glutin::surface::WindowSurface;

use winit::raw_window_handle::HasDisplayHandle;
use winit::raw_window_handle::HasWindowHandle;

use winit::window::Window;

pub struct RenderContext {
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
    glow_context: glow::Context,
    renderer: Renderer,
    texture_map: SimpleTextureMap,
}

impl RenderContext {
    pub async fn new(imgui_context: &mut Context, window: &Window) -> Self {
        let display_handle = window.display_handle().unwrap().as_raw();
        let window_handle = window.window_handle().unwrap().as_raw();

        let display = unsafe {
            #[cfg(target_os = "windows")]
            {
                glutin::display::Display::new(
                    display_handle,
                    glutin::display::DisplayApiPreference::Wgl(Some(window_handle)),
                )
                .or_else(|_| {
                    glutin::display::Display::new(
                        display_handle,
                        glutin::display::DisplayApiPreference::Egl,
                    )
                })
                .expect("Failed to create GL display (WGL/EGL)")
            }

            #[cfg(not(target_os = "windows"))]
            {
                glutin::display::Display::new(
                    display_handle,
                    glutin::display::DisplayApiPreference::Egl,
                )
                .expect("Failed to create GL display (EGL)")
            }
        };

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);
        let config = unsafe {
            display
                .find_configs(template.build())
                .unwrap()
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .expect("No available GL configurations")
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                major: 3,
                minor: 3,
            })))
            .with_profile(glutin::context::GlProfile::Core)
            .build(Some(window_handle));

        let not_current_gl_context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .expect("Failed to create OpenGL context")
        };

        let size = window.inner_size();
        let attrs = glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle,
            NonZeroU32::new(size.width).unwrap_or(NonZeroU32::MIN),
            NonZeroU32::new(size.height).unwrap_or(NonZeroU32::MIN),
        );

        let gl_surface = unsafe {
            display
                .create_window_surface(&config, &attrs)
                .expect("Failed to create GL window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Failed to make OpenGL context current");

        gl_surface
            .set_swap_interval(
                &gl_context,
                glutin::surface::SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
            )
            .expect("Failed to set swap interval (V-Sync)");

        let glow_context = unsafe {
            glow::Context::from_loader_function(|s| {
                let symbol = std::ffi::CString::new(s).unwrap();
                display.get_proc_address(symbol.as_c_str()).cast()
            })
        };
        unsafe {
            glow_context.enable(glow::FRAMEBUFFER_SRGB);
            glow_context.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            glow_context.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
        }

        let mut texture_map = SimpleTextureMap::default();

        let renderer = Renderer::new(&glow_context, imgui_context, &mut texture_map, false)
            .expect("Failed to initialize ImGui Glow renderer");

        Self {
            gl_context,
            gl_surface,
            glow_context,
            renderer,
            texture_map,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let w = NonZeroU32::new(width).unwrap();
            let h = NonZeroU32::new(height).unwrap();
            self.gl_surface.resize(&self.gl_context, w, h);

            unsafe {
                self.glow_context
                    .viewport(0, 0, width as i32, height as i32);
            }
        }
    }

    pub fn render(&mut self, draw_data: &DrawData) {
        unsafe {
            self.glow_context.clear_color(0.1, 0.1, 0.1, 1.0);
            self.glow_context.clear(glow::COLOR_BUFFER_BIT);
        }

        self.renderer
            .render(&self.glow_context, &self.texture_map, draw_data)
            .expect("Failed to render ImGui draw data");

        self.gl_surface
            .swap_buffers(&self.gl_context)
            .expect("Failed to swap buffers");
    }
}
