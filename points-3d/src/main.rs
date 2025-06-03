mod point;
mod draw_context;
mod matrix;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<point::Point>,
}

fn model(app: &App) -> Model {
    let points = vec![
        point::Point::new(vec3(0.0, 0.0, 0.0)),
        point::Point::new(vec3(100.0, 100.0, 0.0)),
    ];

    Model { points }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update logic here
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let draw_context = draw_context::DrawContext::new(&app, &frame);

    for point in &model.points {
        point.draw(&draw_context, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}