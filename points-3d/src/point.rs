use nannou::{glam::Vec3Swizzles, prelude::*};
use crate::draw_context::DrawContext;

pub struct Point {
    position: Vec3,
}

impl Point {
    fn new(position: Vec3) -> Self {
        Point { position }
    }

    fn draw(&self, ctx: &DrawContext, draw: &Draw) {
        draw.ellipse()
            .xy(self.position.xy())
            .radius(2.0)
            .color(BLACK);
    }
}