
use nannou::{draw, prelude::*};
use quadtree::Point;

#[derive(Debug, Clone)]
pub struct Ball {
    id: u32,
    pub position: Point2,
    pub velocity: Vec2,
    pub radius: f32,
    colour: Rgb<u8>,
}

impl Ball {
    pub fn new(xy: Point2, v: Vec2, radius: f32, colour: Rgb<u8>) -> Self {
        Ball { id: random_range(0, u32::MAX), position: xy, velocity: v, radius, colour }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .radius(self.radius)
            .color(self.colour);
    }

    pub fn mass(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

impl Point for Ball {
    fn point(&self) -> quadtree::P2 {
        quadtree::P2::new(self.position.x as f64, self.position.y as f64)
    }
}