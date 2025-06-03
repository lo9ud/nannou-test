use crate::Hole;

use super::Drawable;
use nannou::prelude::*;
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
}

impl Drawable for Particle {
    fn draw(&self, draw: &nannou::draw::Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.mass.sqrt() / 2.0)
            .color(WHITE);
    }
}
impl Particle {
    pub fn _get_grav_force(&self, hole: &Hole) -> Vec2 {
        let direction = hole.position - self.position;
        let distance = direction.length();
        let grav_force = direction.normalize() / distance.powi(2) * hole.mass * self.mass;
        grav_force
    }

    pub fn _get_rot_force(&self, hole: &Hole) -> Vec2 {
        let direction = hole.position - self.position;
        let distance = direction.length();
        let rot_force = vec2(-direction.y, direction.x).normalize() / distance.powi(2)
            * hole.mass
            * hole.rotation;
        rot_force
    }

    pub fn update(&mut self, app: &App, dt: f32, holes: &Vec<Hole>) {
        self.position += self.velocity * dt;

        for hole in holes {
            self.velocity +=
                (self._get_grav_force(hole) + self._get_rot_force(hole)) / self.mass * dt;
        }
    }
}
