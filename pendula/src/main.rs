use nannou::prelude::*;
use pendulum::Pendulum;
use style::{ArmStyle, HingeStyle, Style, TipStyle};

mod pendulum;
mod style;


const MAX_UPDATE: f32 = 0.01;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    pendulums: Vec<pendulum::Pendulum>,
}

fn model(_app: &App) -> Model {
    let pendulums = vec![
        Pendulum {
            length: 100.0,
            angle: 0.1,
            speed: 0.1,
            style: Style {
                arm: ArmStyle::Linear(rgba(255,0,0,100), rgba(0,255,0,175)),
                tip: TipStyle::None,
                hinge: HingeStyle::None,
            }
        },
        Pendulum {
            length: 150.0,
            angle: 0.2,
            speed: -0.15,
            style: Style {
                arm: ArmStyle::Linear(rgba(0,255,0,175), rgba(0,0,255,100)),
                tip: TipStyle::None,
                hinge: HingeStyle::None,
            }
        }
    ];

    Model {
        pendulums,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last;
    for p in model.pendulums.iter_mut() {
        p.update(dt);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // draw.background().color(rgb8(35, 35,35));

    let mut root = vec2(0.0,0.0);
    for p in &model.pendulums {
        root = p.draw_from(&draw, root);
    };
    draw.to_frame(app, &frame).unwrap();
}
