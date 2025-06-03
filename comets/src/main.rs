extern crate nannou;
mod comet;

use nannou::prelude::*;
const SPEED: u32 = 20;
const TIME_STEP: f32 = 1.0 / SPEED as f32;
const COMET_COUNT: u32 = 128;
const TRAIL_FADE: f32 = 0.01;
const COMET_BEHAVIOUR: comet::CometBehaviour = comet::CometBehaviour {
    color: nannou::color::Alpha {
        color: Rgb {
            red: 0.8,
            green: 0.8,
            blue: 0.8,
            standard: std::marker::PhantomData,
        },
        alpha: 1.0,
    },

    // forces
    central_tendency: 0.0,
    gravity: 9.0,
    friction: 0.05,
    mass: 1.0,
    rotation: 30.0,

    // comet
    width: 2.0,
    length: 4.0,
};
fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    comets: Vec<comet::Comet>,
}

fn model(_app: &App) -> Model {
    // Create the comets
    let mut comets: Vec<comet::Comet> = vec![];
    for i in 0..COMET_COUNT {
        comets.push(comet::Comet::new(
            Point2::new(1.0, 0.0).rotate(PI * 2.0 * (i as f32 / COMET_COUNT as f32)) * 200.0,
            Vec2::new(0.0, 1.0).rotate(PI * 2.0 * (i as f32 / COMET_COUNT as f32))*20.0,
            COMET_BEHAVIOUR,
        ));
    }
    // Return the model
    Model { comets }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(_app: &App, model: &mut Model, update: Update) {
    let delta = update.since_last.secs() / TIME_STEP as f64;

    let mut comets: Vec<comet::Comet> = vec![];
    for comet in &model.comets {
        let mut new_comet = *comet;
        new_comet.update(delta, &model.comets);
        comets.push(new_comet)
    }

    model.comets = comets;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .wh(app.window_rect().wh())
        .xy(app.window_rect().xy())
        .color(rgba(
            0.0,
            0.0,
            0.0,
            1.0 - 1.0 / (1.0 + TRAIL_FADE * SPEED as f32),
        ));

    for comet in model.comets.iter() {
        comet.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
