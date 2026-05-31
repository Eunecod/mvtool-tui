// libs/mvframe/src/backend/canvas.rs

use imgui::Context;
use imgui::DrawData;

use wgpu::Color;
use wgpu::CommandEncoderDescriptor;
use wgpu::Device;
use wgpu::DeviceDescriptor;
use wgpu::Instance;
use wgpu::LoadOp;
use wgpu::Operations;
use wgpu::PowerPreference;
use wgpu::Queue;
use wgpu::RenderPassColorAttachment;
use wgpu::RenderPassDescriptor;
use wgpu::RequestAdapterOptions;
use wgpu::StoreOp;
use wgpu::Surface;
use wgpu::SurfaceConfiguration;
use wgpu::TextureUsages;
use wgpu::TextureViewDescriptor;

use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;

use winit::window::Window;

pub struct RenderContext {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    renderer: Renderer,
}

impl RenderContext {
    pub async fn new(context: &mut Context, window: &Window) -> Self {
        let instance = Instance::default();

        let window_static: &'static Window = unsafe { std::mem::transmute(window) };
        let surface = instance
            .create_surface(window_static)
            .expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to request adapter");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Mvframe Render Device"),
                ..Default::default()
            })
            .await
            .expect("Failed to request device");

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps.formats[0];

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let config = RendererConfig {
            texture_format,
            ..RendererConfig::default()
        };
        let renderer = Renderer::new(context, &device, &queue, config);

        Self {
            surface,
            surface_config,
            device,
            queue,
            renderer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;

            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn render(&mut self, draw_data: &DrawData) {
        let current_texture = self.surface.get_current_texture();

        let surface_texture = match current_texture {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            _ => return,
        };
        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Mvframe Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Mvframe UI Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });

            let _ = self
                .renderer
                .render(draw_data, &self.queue, &self.device, &mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}
