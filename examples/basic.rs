use particulo::{engine::SimulationContext, Engine, Particle};

#[derive(Debug)]
struct Custom {
    mass: f64,
}

struct Context {}

fn user_fn(context: SimulationContext<Context, Custom>) -> anyhow::Result<()> {
    Ok(())
}

fn main() {
    let particles = std::iter::from_fn(|| {
        let mut p = Particle::new(Custom { mass: 0.0 });
        p.randomize_pos();
        Some(p)
    })
    .take(100);

    let mut engine = Engine::new(Context {});
    engine.add_particles(particles);
    engine.set_user_fn(user_fn);
    engine.start().unwrap();
}
