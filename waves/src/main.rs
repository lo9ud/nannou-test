use std::time::Instant;
use paste::paste;

use nannou::prelude::*;
use rayon::prelude::*;

const COUNT: usize = 100;
const MASS: f32 = 100.0;
const CENTRAL_FORCE: f32 = 0.0;
const FRICTION: f32 = 0.5;
const BOND_FORCE: f32 = 2.0;

macro_rules! do_time {
    {$id:ident, $tt:block} => {
        paste! {
            let [<$id _duration>] = std::time::Instant::now();
            $tt
            let duration = [<$id _duration>].elapsed();
            println!(concat!("Time taken for ", stringify!($id), ": {:#?}"), duration);
        }
    };
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: WindowId,
    last_update: Instant,
    particles: Vec<Particle>,
}

struct Particle {
    position: f32,
    velocity: f32,
}

impl Particle {
    fn new() -> Self {
        Particle {
            position: 0.0,
            velocity: 0.0,
        }
    }

    fn update(&mut self, delta: f32, neighbours: &[Option<&f32>; 2]) {
        let raw_tension = neighbours.iter().filter_map(|&n| n).map(|position| position - self.position).sum::<f32>();
        let tension = raw_tension * BOND_FORCE;
        let damping = -self.velocity * FRICTION;
        let central_force = -self.position * CENTRAL_FORCE;

        let acceleration = tension + damping + central_force / MASS;
        self.velocity += acceleration * delta;
        self.position += self.velocity * delta;
    }

    fn perturb(&mut self) {
        let perturbation = ((random::<f32>() - 0.5) * 2.0).signum() * 1.5;
        self.velocity = perturbation;
        // self.velocity = 1.0;
    }

    fn clamp(&mut self) {
        if self.position < -1.0 {
            self.position = -1.0;
        } else if self.position > 1.0 {
            self.position = 1.0;
        }
    }
}

fn model(app: &App) -> Model {
    let window = app.new_window()
        .size(800, 800)
        .view(view)
        .msaa_samples(4)
        .build()
        .unwrap();

    let mut particles = (0..COUNT).map(|_| Particle::new()).collect::<Vec<_>>();
    particles.get_mut(random_range(0, COUNT)).map(|particle| {
        particle.perturb();
    });

    Model {
        _window: window,
        last_update: Instant::now(),
        particles,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    do_time!(update, {
        let inner_positions = model.particles.iter().map(|p| p.position).collect::<Vec<_>>();
        let positions = vec![0.0].into_iter().chain(inner_positions).chain(vec![0.0]).collect::<Vec<_>>();
        let n_particles = model.particles.len();
        let delta = model.last_update.elapsed().as_secs_f32()/100.0;

        // update particles
        model.particles.par_iter_mut().enumerate().for_each(|(i, particle)| {
            let left = Some(&positions[i]);
            let right = Some(&positions[i + 2]);
            let neighbours = [left, right];
            particle.update(delta, &neighbours);
            particle.clamp();
        });

        // perturb particles
        if app.elapsed_frames() % 75 == 0 {
            let random_index = random_range(0, n_particles);
            model.particles.get_mut(random_index).map(|particle| {
                particle.perturb();
            });
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    do_time!(draw, {
        let draw = app.draw();
        draw.background().color(BLACK);

        let bounds = frame.rect();

        // bonds
        for (i, ab) in model.particles.windows(2).enumerate() {
            let a = &ab[0];
            let b = &ab[1];
            let a_pos = vec2(map_range(i, 0, COUNT, bounds.left(), bounds.right()), map_range(a.position, -1.0, 1.0, bounds.bottom(), bounds.top()));
            let b_pos = vec2(map_range(i + 1, 0, COUNT, bounds.left(), bounds.right()), map_range(b.position, -1.0, 1.0, bounds.bottom(), bounds.top()));
            draw.line()
                .start(a_pos)
                .end(b_pos)
                .color(WHITE);
        }

        // particles
        for (i, particle) in model.particles.iter().enumerate() {
            let x = map_range(i as f32, 0.0, model.particles.len() as f32, bounds.left(), bounds.right());
            let y = map_range(particle.position, -1.0, 1.0, bounds.bottom(), bounds.top());
            draw.ellipse()
                .x_y(x, y)
                .radius(bounds.w()/(2.1 * COUNT as f32))
                .color(WHITE);
        }

        draw.to_frame(app, &frame).unwrap();
    });
}

