use std::time::Instant;

use wgpu::{Buffer, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::Engine;

use super::SimulationContext;

impl<Ctx: 'static, C: 'static> Engine<Ctx, C> {
    pub(crate) fn sim_loop(
        mut self,
        event_loop: EventLoop<()>,
        window: Window,
        surface: Surface,
        device: Device,
        queue: Queue,
        render_pipeline: RenderPipeline,
        vertex_buffer: Buffer,
        mut config: SurfaceConfiguration,
    ) -> anyhow::Result<()> {
        let user_fn = self.user_fn.take();
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                config.width = if size.width == 0 { 1 } else { size.width };
                config.height = if size.height == 0 { 1 } else { size.height };
                surface.configure(&device, &config);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                self.dt = self.prev_time.elapsed();
                self.prev_time = Instant::now();

                if let Some(ref user_fn) = user_fn {
                    user_fn(SimulationContext {
                        dt: self.dt,
                        start: self.start,
                        particles: &mut self.particles,
                        custom: &mut self.custom,
                    })
                    .unwrap();
                }

                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.draw(0..6, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        });
    }
}
