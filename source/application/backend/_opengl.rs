// source/application/backend/_opengl.rs

use sdl2::VideoSubsystem;

use imgui_glow_renderer::AutoRenderer;

pub struct OpenGL {
    gl_renderer: AutoRenderer
}

impl OpenGL {
	pub fn new(video_subsystem: &VideoSubsystem, imgui_context: &mut imgui::Context) -> Result<Self, String> {
		let context = unsafe {
            glow::Context::from_loader_function(|system| {
                video_subsystem.gl_get_proc_address(system) as *const _
            })
        };

        let gl_renderer = AutoRenderer::new(context, imgui_context).map_err(|error| error.to_string())?;

	    Ok(Self { gl_renderer: gl_renderer })
    }

    pub fn gl_renderer(&mut self) -> &mut AutoRenderer {
        &mut self.gl_renderer
    }
}