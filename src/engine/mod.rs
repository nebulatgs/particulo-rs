use std::{
    pin::Pin,
    time::{Duration, Instant},
};

use crate::Particle;

mod init_context;
mod init_window;
mod sim_loop;

pub struct Engine<Ctx, C> {
    pub dt: Duration,
    pub start: Instant,

    pub custom: Ctx,

    prev_time: Instant,

    particles: Vec<Particle<C>>,

    user_fn: Option<Pin<Box<fn(SimulationContext<Ctx, C>) -> anyhow::Result<()>>>>,
}

pub struct SimulationContext<'a, Ctx, C> {
    pub dt: Duration,
    pub start: Instant,
    pub particles: &'a mut [Particle<C>],
    pub custom: &'a mut Ctx,
}

impl<Ctx: 'static, C: 'static> Engine<Ctx, C> {
    pub fn new(custom: Ctx) -> Engine<Ctx, C> {
        Engine {
            particles: Vec::new(),
            dt: Duration::default(),
            custom,

            start: Instant::now(),
            prev_time: Instant::now(),

            user_fn: None,
        }
    }

    pub fn set_user_fn(&mut self, user_fn: fn(SimulationContext<Ctx, C>) -> anyhow::Result<()>) {
        let user_fn = Box::pin(user_fn);
        self.user_fn = Some(user_fn);
    }

    pub fn add_particles(
        &mut self,
        particles: impl IntoIterator<Item = Particle<C>, IntoIter = impl Iterator<Item = Particle<C>>>,
    ) {
        self.particles.extend(particles);
    }

    pub fn start(mut self) -> anyhow::Result<()> {
        let (window, event_loop) = self.init_window()?;

        let (surface, device, queue, render_pipeline, config, vertex_buffer) =
            pollster::block_on(self.init_context(&window))?;

        self.start = Instant::now();
        self.sim_loop(
            event_loop,
            window,
            surface,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            config,
        )?;
        Ok(())
    }
}
