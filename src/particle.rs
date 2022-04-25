use nalgebra::Vector2;
use palette::rgb::Rgba;

#[derive(Default, Debug)]
pub struct Particle<C = ()> {
    pub cur_pos: Vector2<f64>,
    pub prev_pos: Vector2<f64>,
    pub acc: Vector2<f64>,
    pub radius: f64,
    pub color: Rgba,
    pub custom: C,
}

impl<C> Particle<C> {
    pub fn new(custom: C) -> Self {
        Particle {
            cur_pos: Vector2::new(0.0, 0.0),
            prev_pos: Vector2::new(0.0, 0.0),
            acc: Vector2::new(0.0, 0.0),
            radius: 0.0,
            color: Rgba::new(0.0, 0.0, 0.0, 0.0),
            custom,
        }
    }
    pub fn randomize_pos(&mut self) {
        self.cur_pos.x = rand::random::<f64>();
        self.cur_pos.y = rand::random::<f64>();
    }
}
