/// A module for the BoidBehaviour struct
pub mod behaviour {
    use nannou::prelude::vec2;

    #[derive(Debug, Clone)]
    pub struct BoidBehaviour {
        pub debug: bool,

        pub group: u64,
        pub color: nannou::color::Rgb<u8>,
        pub size: f32,
        pub points: Vec<nannou::geom::Point2>,

        pub inertia: f32,
        pub alignment: f32,
        pub cohesion: f32,
        pub separation: f32,
        pub avoidance: f32,
        pub mouse_bias: f32,

        pub neighbourhood: f32,
        pub personal_space: f32,

        pub speed: f32,
    }

    impl Default for BoidBehaviour {
        fn default() -> Self {
            BoidBehaviour {
                debug: false,

                group: 0,
                color: nannou::color::rgb(255, 255, 255),
                size: 1.0,
                points: vec![
                    vec2(0.0, 10.0),
                    vec2(5.0, -5.0),
                    vec2(0.0, -2.0),
                    vec2(-5.0, -5.0),
                ],
                inertia: 0.9,
                alignment: 0.1,
                cohesion: 0.1,
                separation: 0.1,
                avoidance: 0.1,
                mouse_bias: 0.1,
                neighbourhood: 150.0,
                personal_space: 50.0,
                speed: 2.0,
            }
        }
    }

    impl BoidBehaviour {
        pub fn weight(&self) -> f32 {
            self.alignment + self.cohesion + self.separation
        }

        pub fn random() -> Self {
            let neighbourhood_radius = (80.0 + 200.0 * rand::random::<f32>()).max(150.0);
            let personal = (neighbourhood_radius / (2.0 + 3.0 * rand::random::<f32>())).min(100.0);
            BoidBehaviour {
                debug: false,
                group: rand::random::<u64>(),
                color: nannou::color::rgb(
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                ),
                size: (1.0 + rand::random::<f32>() * neighbourhood_radius / personal)
                    .min(2.0)
                    .max(1.0),

                points: if rand::random::<bool>() {
                    vec![
                        vec2(0.0, 10.0),
                        vec2(5.0, -5.0),
                        vec2(0.0, -2.0),
                        vec2(-5.0, -5.0),
                    ]
                } else {
                    vec![
                        vec2(0.0, 10.0),
                        vec2(2.0, 8.0),
                        vec2(3.0, 4.0),
                        vec2(0.0, 0.0),
                        vec2(2.0, -3.0),
                        vec2(-2.0, -3.0),
                        vec2(0.0, 0.0),
                        vec2(-3.0, 4.0),
                        vec2(-2.0, 8.0),
                    ]
                },

                inertia: rand::random::<f32>().max(0.5),
                alignment: 0.4 + 0.2 * rand::random::<f32>(),
                cohesion: 0.5,
                separation: 0.5 + 0.1 * rand::random::<f32>(),
                avoidance: rand::random::<f32>().max(0.5),
                mouse_bias: rand::random::<f32>().max(0.5),

                neighbourhood: neighbourhood_radius,
                personal_space: personal,
                speed: 1.5 + 2.5 * rand::random::<f32>(),
            }
        }
    }
}

/// A module for the Boid struct
pub mod boid {
    use super::behaviour::BoidBehaviour;
    use nannou::{
        prelude::{vec2, Rect, Vec2},
        App,
    };
    const VECTOR_SCALE: f32 = 10.0;
    #[derive(Debug)]
    pub struct Boid {
        id: usize,

        pub behaviour: BoidBehaviour,

        pub position: Vec2,
        pub velocity: Vec2,

        pub alignment_vec: Vec2,
        pub cohesion_vec: Vec2,
        pub separation_vec: Vec2,
        pub avoidance_vec: Vec2,

        pub bias: Vec2,
    }

    impl Clone for Boid {
        fn clone(&self) -> Self {
            Boid {
                id: self.id,
                behaviour: self.behaviour.clone(),
                position: self.position,
                velocity: self.velocity,
                alignment_vec: self.alignment_vec,
                cohesion_vec: self.cohesion_vec,
                separation_vec: self.separation_vec,
                avoidance_vec: self.avoidance_vec,
                bias: self.bias,
            }
        }
    }

    impl Boid {
        pub fn new(behaviour: BoidBehaviour, position: Vec2, velocity: Vec2) -> Self {
            Boid {
                id: rand::random::<usize>(),
                behaviour,
                position,
                velocity,
                alignment_vec: vec2(0.0, 0.0),
                cohesion_vec: vec2(0.0, 0.0),
                separation_vec: vec2(0.0, 0.0),
                avoidance_vec: vec2(0.0, 0.0),
                bias: vec2(0.0, 0.0),
            }
        }

        pub fn draw_debug(&self, draw: &nannou::draw::Draw) {
            draw.ellipse()
                .x_y(self.position.x, self.position.y)
                .w_h(
                    self.behaviour.personal_space * 2.0,
                    self.behaviour.personal_space * 2.0,
                )
                .stroke(nannou::color::GRAY)
                .stroke_weight(1.0)
                .no_fill();

            draw.ellipse()
                .x_y(self.position.x, self.position.y)
                .w_h(
                    self.behaviour.neighbourhood * 2.0,
                    self.behaviour.neighbourhood * 2.0,
                )
                .stroke(nannou::color::GRAY)
                .stroke_weight(1.0)
                .no_fill();

            draw.arrow()
                .start(self.position)
                .end(self.position + self.alignment_vec * VECTOR_SCALE)
                .weight(2.0)
                .head_length(5.0)
                .head_width(5.0)
                .color(nannou::color::RED);

            draw.arrow()
                .start(self.position)
                .end(self.position + self.cohesion_vec * VECTOR_SCALE)
                .weight(2.0)
                .head_length(5.0)
                .head_width(5.0)
                .color(nannou::color::GREEN);

            draw.arrow()
                .start(self.position)
                .end(self.position + self.separation_vec * VECTOR_SCALE)
                .weight(2.0)
                .head_length(5.0)
                .head_width(5.0)
                .color(nannou::color::BLUE);

            draw.arrow()
                .start(self.position)
                .end(self.position + self.avoidance_vec * VECTOR_SCALE)
                .weight(2.0)
                .head_length(5.0)
                .head_width(5.0)
                .color(nannou::color::YELLOW);

            draw.arrow()
                .start(self.position)
                .end(self.position + self.bias * VECTOR_SCALE)
                .weight(2.0)
                .head_length(5.0)
                .head_width(5.0)
                .color(nannou::color::PURPLE);

            self._draw(draw);
        }

        fn _draw(&self, draw: &nannou::draw::Draw) {
            draw.polygon()
                .points(
                    self.behaviour
                        .points
                        .iter()
                        .cloned()
                        .map(|p| p * self.behaviour.size)
                        .collect::<Vec<Vec2>>(),
                )
                .x_y(self.position.x, self.position.y)
                .rotate(-self.velocity.angle_between(vec2(0.0, 1.0)))
                .color(self.behaviour.color);
        }

        pub fn draw(&self, draw: &nannou::draw::Draw) {
            if self.behaviour.debug {
                self.draw_debug(draw);
            } else {
                self._draw(draw);
            }
        }
        /// Calculate the alignment vector for a boid
        /// Alignment is the vector that points in the average direction of the local boids
        fn align(&self, boids: &Vec<Boid>) -> Vec2 {
            let mut alignment = vec2(0.0, 0.0);
            for boid in boids {
                if self.behaviour.group == boid.behaviour.group {
                    alignment += boid.velocity / self.behaviour.speed.powf(boids.len() as f32);
                }
            }
            return alignment;
        }

        /// Calculate the clustering vector for a boid
        /// Cohesion is the vector that points towards the center of mass of the local boids of the same group
        fn cohere(&self, boids: &Vec<Boid>, frame: &Rect) -> Vec2 {
            let mut coherence = vec2(0.0, 0.0);
            for boid in boids {
                if boid.behaviour.group == self.behaviour.group {
                    let vec = self.get_vector_to(boid, &frame);
                    let distance = vec.length();
                    coherence += (boid.position - self.position).normalize()
                        * (distance / (self.behaviour.neighbourhood * boids.len() as f32)).powi(3);
                }
            }
            return coherence;
        }

        /// Calculate the separation vector for a boid
        /// Separation is the vector that points away from the closest local boid
        fn separate(&self, boids: &Vec<Boid>, frame: &Rect) -> Vec2 {
            let mut separation = vec2(0.0, 0.0);
            for boid in boids {
                let vec = self.get_vector_to(boid, &frame);
                let distance = vec.length();
                separation += vec.normalize()
                    * (1f32 / (distance - self.behaviour.personal_space).max(1f32 / 5f32));
            }

            return separation;
        }

        /// Calculate the bias vector for a boid
        /// Bias is the vector that points towards the mouse
        fn bias(&self, app: &App) -> Vec2 {
            let mouse = app.mouse.position();
            if app.mouse.buttons.left().is_down() {
                return (mouse - self.position).normalize();
            } else if app.mouse.buttons.right().is_down() {
                return (self.position - mouse).normalize();
            }
            return vec2(0.0, 0.0);
        }

        pub fn update(&mut self, app: &App, boids: &Vec<Boid>) {
            let frame = app.window_rect();

            // Wrap around the screen
            self.position += self.velocity;
            if self.position.x < frame.left() - 15.0 {
                self.position.x = frame.right() - 1.0;
            } else if self.position.x > frame.right() + 15.0 {
                self.position.x = frame.left() + 1.0;
            }
            if self.position.y < frame.bottom() - 15.0 {
                self.position.y = frame.top() - 1.0;
            } else if self.position.y > frame.top() + 15.0 {
                self.position.y = frame.bottom() + 1.0;
            }

            // Get the local boids
            let boids: Vec<Boid> = boids
                .iter()
                .filter(|boid| {
                    self.position.distance(boid.position) < self.behaviour.neighbourhood
                        && boid.id != self.id // Don't include self
                })
                .map(|boid| boid.clone())
                .collect();

            // Calculate the vectors
            if boids.len() > 0 {
                self.alignment_vec = self.align(&boids);
                self.cohesion_vec = self.cohere(&boids, &frame);
                self.separation_vec = self.separate(&boids, &frame);
            } else {
                self.alignment_vec = vec2(0.0, 0.0);
                self.cohesion_vec = vec2(0.0, 0.0);
                self.separation_vec = vec2(0.0, 0.0);
            }
            // self.avoidance_vec = self.avoid(app);
            self.bias = self.bias(app);

            // Update the velocity
            self.velocity = (self.velocity.normalize_or_zero() * self.behaviour.inertia
                + self.alignment_vec.normalize_or_zero() * self.behaviour.alignment
                + self.cohesion_vec.normalize_or_zero() * self.behaviour.cohesion
                + self.separation_vec.normalize_or_zero() * self.behaviour.separation
                // + self.avoidance_vec.normalize_or_zero() * self.behaviour.avoidance
                + self.bias.normalize_or_zero() * self.behaviour.mouse_bias)
                // .normalize_or_zero()
                * self.behaviour.speed
                / self.behaviour.weight();
        }

        fn get_vector_to(&self, to: &Boid, frame: &Rect) -> Vec2 {
            let d_y = self.position.y - to.position.y;
            let d_y = if d_y > frame.h() / 2.0 {
                d_y - frame.h()
            } else if d_y < -frame.h() / 2.0 {
                d_y + frame.h()
            } else {
                d_y
            };
            let d_x = self.position.x - to.position.x;
            let d_x = if d_x > frame.w() / 2.0 {
                d_x - frame.w()
            } else if d_x < -frame.w() / 2.0 {
                d_x + frame.w()
            } else {
                d_x
            };
            vec2(d_x, d_y)
        }
    }
}
