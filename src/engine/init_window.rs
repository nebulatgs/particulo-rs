use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::Engine;

impl<Ctx, C> Engine<Ctx, C> {
    pub(crate) fn init_window(&mut self) -> anyhow::Result<(Window, EventLoop<()>)> {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop)?;

        Ok((window, event_loop))
    }
}
