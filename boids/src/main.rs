mod boids;

use boids::{behaviour::BoidBehaviour, boid::Boid};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug)]
struct Model {
    boids: Vec<Boid>,
}

fn model(_app: &App) -> Model {
    // let groups: Vec<(BoidBehaviour, u8)> = vec![
    //     (BoidBehaviour::default(), 8),
    //     (BoidBehaviour::random(), 12),
    //     (BoidBehaviour::random(), 8),
    //     (BoidBehaviour::random(), 8),
    //     (BoidBehaviour::random(), 8),
    //     (BoidBehaviour::random(), 8),
    //     (BoidBehaviour::random(), 12),
    //     (BoidBehaviour::random(), 24),
        // (
        //     BoidBehaviour {
        //         group: 1,
        //         color: rgb(255u8, 0u8, 0u8),
        //         speed: 3.0,
        //         inertia: 1.0,
        //         avoidance: 1.0,
        //         separation: 4.0,
        //         alignment: 3.0,
        //         cohesion: 1.0,
        //         mouse_bias: 1.0,
        //         neighbourhood: 300.0,
        //         personal_space: 5.0,
        //     },
        //     8,
        // ),
        // (
        //     BoidBehaviour {
        //         group: 2,
        //         color: rgb(0u8, 255u8, 0u8),
        //         speed: 1.0,
        //         inertia: 5.0,
        //         avoidance: 1.0,
        //         separation: 2.0,
        //         alignment: 4.0,
        //         cohesion: 2.0,
        //         mouse_bias: 1.0,
        //         neighbourhood: 200.0,
        //         personal_space: 75.0,
        //     },
        //     8,
        // ),
    // ];

    let r = 500.0;
    let mut boids: Vec<Boid> = vec![];
    // for (behaviour, n) in groups {
    for _i in 0..12 {
        let behaviour = BoidBehaviour::random();
        let n = (rand::random::<f32>() * 12f32 + 8f32).round() as u32;
        for i in 0..n {
            let theta = i as f32 * PI * 2.0 / n as f32;
            let r_theta = rand::random::<f32>() * 2.0 * PI;
            let unit = pt2((theta).cos(), (theta).sin());
            boids.push(Boid::new(
                behaviour.clone(),
                r * unit * (rand::random::<f32>() * 0.8 + 0.8),
                pt2(r_theta.cos(), r_theta.sin()).normalize(),
            ));
        }
    }
    return Model { boids: boids };
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut new_boids: Vec<Boid> = vec![];
    for boid in &model.boids {
        let mut new_boid = boid.clone();
        new_boid.update(app, &model.boids);
        new_boids.push(new_boid);
    }
    model.boids = new_boids;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb(20u8, 20u8, 20u8));
    for boid in &model.boids {
        boid.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
