use std::time::Duration;

use nannou::{glam::{vec2, Vec2}, math::Vec2Rotate, Draw};

use crate::style::Style;

pub struct Pendulum {
    pub angle: f32,
    pub length: f32,
    /// Radians per second
    pub speed: f32,
    pub style: Style,
}

impl Pendulum {
    pub fn draw_from(&self, draw: &Draw, from: Vec2) -> Vec2 {
        let tip = from + vec2(0.0, self.length).rotate(self.angle);
        self.style.draw(draw, from, tip);
        tip
    }

    pub fn update(&mut self, dt: Duration) {
        self.angle += self.speed * dt.as_secs_f32();
    }
}