mod ant;
mod map;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    pub ants: Vec<ant::Ant>,
    pub map: map::Map,
}

fn model(app: &App) -> Model {
    let map = map::Map::new(10, 10);
    let ants = (0..10).map(|_| ant::Ant::new(nannou::prelude::vec2(0.0, 0.0))).collect();
    Model { ants, map }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for ant in model.ants.iter_mut() {
        ant.update(&model.map);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for tile in model.map.tiles().iter() {
        let color = match tile.peek() {
            Some(map::Resource::Food) => RED,
            None => WHITE,
        };
        draw.rect()
            .xy(nannou::prelude::vec2(0.0, 0.0))
            .wh(nannou::prelude::vec2(1.0, 1.0))
            .color(color);
    }

    for ant in model.ants.iter() {
        draw.ellipse()
            .xy(ant.position())
            .radius(1.0)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
