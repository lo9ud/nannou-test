extern crate nannou;
use std::fmt::Display;

use nannou::{
    draw,
    geom::{Point2, Vec2},
    prelude::*,
};

#[derive(Debug, Clone, Copy)]
pub struct CometBehaviour {
    // What an comet is
    /// The color of the comet (used to draw it on the screen and to color the trail it leaves behind)
    pub color: Rgba<f32>,
    /// The width of the comet (Across its direction of travel)
    pub width: f32,
    /// The length of the comet (Along its direction of travel)
    pub length: f32,

    // How an comet behaves
    /// The central tendency force is a force that pulls the comet towards the center of the screen
    pub central_tendency: f32,
    /// The gravity force is a force that pulls the comet towards other comets
    pub gravity: f32,
    /// The friction force is a force that slows the comet down
    pub friction: f32,
    /// The mass of the comet
    pub mass: f32,
    /// The rotation force is a force that rotates the comet around the center of the screen
    pub rotation: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Comet {
    id: u32,
    pub position: Point2,
    last: Point2,
    pub velocity: Vec2,
    pub behaviour: CometBehaviour,
}

impl Display for Comet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Comet {{position: {}, velocity: {}}}",
            self.position, self.velocity,
        )
    }
}

impl Comet {
    pub fn new(position: Point2, velocity: Vec2, behaviour: CometBehaviour) -> Self {
        Comet {
            id: random_range(0, u32::MAX),
            position,
            velocity,
            behaviour,
            last: position,
        }
    }
    pub fn draw(&self, draw: &draw::Draw) {
        draw.line()
            .start(self.last)
            .end(self.position)
            .stroke_weight(self.behaviour.width)
            .caps_round()
            .color(self.behaviour.color);
    }

    pub fn update(&mut self, delta: f64, comets: &[Comet]) {
        if self.position.length() > 300.0 {
            self.position = self.position.normalize() * 299.8;
            self.velocity = self.velocity - self.velocity.project_onto(self.position.normalize());
            return;
        }
        let mut comets = comets
            .iter()
            .filter(|comet| comet.id != self.id)
            .collect::<Vec<_>>();
        comets.sort_by(|a, b| {
                let distance_a = (a.position - self.position).length();
                let distance_b = (b.position - self.position).length();
                distance_a.partial_cmp(&distance_b).unwrap()
            });
        comets = comets.iter().take(6).cloned().collect::<Vec<_>>();
        let attraction_force = comets
            .iter()
            .filter(|comet| comet.id != self.id)
            .map(|comet| {
                let distance = (comet.position - self.position).length();
                let direction = (comet.position - self.position).normalize_or_zero();
                direction * distance.recip().min(f32::MAX) * comet.behaviour.mass
            })
            .fold(Vec2::new(0.0, 0.0), |acc, force| acc + force);

        let friction_force = self.velocity * -1.0;
        let central_tendency_force = -self.position.normalize();
        let rotation_force = self.position.perp().normalize() * self.position.length_recip();

        self.velocity += (
            // Calculate the acceleration
            attraction_force * self.behaviour.gravity
                + friction_force * self.behaviour.friction
                + central_tendency_force * self.behaviour.central_tendency
                + rotation_force * self.behaviour.rotation
        ) / self.behaviour.mass
            * delta as f32;
        self.last = self.position;
        self.position += self.velocity * delta as f32;
    }
}
