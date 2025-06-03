use super::Drawable;
use nannou::prelude::*;
pub struct Hole {
    pub position: Vec2,
    pub mass: f32,
    pub rotation: f32,
}

impl Drawable for Hole {
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .radius(self.mass.sqrt() / 2.0)
            .color(BLACK);
    }
}
