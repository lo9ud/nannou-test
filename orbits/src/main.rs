mod bubble;

use nannou::prelude::*;

fn main() {
    println!("Hello, world!");
}

fn model(app: &App) -> Model {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model{
    bubbles: Vec<Bubble>,
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(BLACK);
}

fn window_event(_app: &App, _model: &Model, _event: WindowEvent) {
}


