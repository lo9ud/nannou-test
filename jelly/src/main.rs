mod jelly;

use jelly::Jelly;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    jellies: Vec<Jelly>,
    jelly_positions: Vec<f32>,
}

fn model(_app: &App) -> Model {
    Model {
        jellies: vec![Jelly::default()],
        jelly_positions: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for jelly in model.jellies.iter_mut() {
        jelly.update();
    }
    if model.jelly_positions.len() < 100 {
        model
            .jelly_positions
            .push(model.jellies[0].velocity.length());
    } else {
        model.jelly_positions.remove(0);
        model
            .jelly_positions
            .push(model.jellies[0].velocity.length());
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            KeyPressed(Key::Space) => {
                model.jellies.push(Jelly::default());
            }
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for jelly in model.jellies.iter() {
        jelly.draw(&draw);
    }

    let win = app.window_rect();
    
    // velocity graph
    let velocity_graph_rect = Rect::from_w_h(200.0, 200.0)
        .bottom_left_of(win.pad(20.0));
    let mut points: Vec<Vec2> = Vec::new();
    let max_y = model
        .jelly_positions
        .iter()
        .fold(0.0, |acc, &x| acc.max(x));
    for (i, pos) in model.jelly_positions.iter().enumerate() {
        let x = map_range(
            i,
            0,
            100,
            velocity_graph_rect.left(),
            velocity_graph_rect.right(),
        );
        let y = map_range(
            pos.to_owned(),
            0.0,
            max_y,
            velocity_graph_rect.bottom(),
            velocity_graph_rect.top(),
        );
        points.push(pt2(x, y));
    }
    draw.line()
        .start(velocity_graph_rect.bottom_left())
        .end(velocity_graph_rect.top_left())
        .weight(1.0)
        .color(GRAY);
    draw.polyline().weight(2.0).points(points).color(RED);
    draw.rect()
        .no_fill()
        .xy(velocity_graph_rect.xy())
        .wh(velocity_graph_rect.wh())
        .stroke(WHITE)
        .stroke_weight(1.0);

    // direction graph
    let direction_graph_rect = Rect::from_w_h(200.0, 200.0)
        .right_of(velocity_graph_rect)
        .align_top_of(velocity_graph_rect);
    draw.arrow()
        .start(direction_graph_rect.xy())
        .end(direction_graph_rect.xy() + model.jellies[0].velocity.normalize() * 80.0)
        .color(GRAY)
        .weight(1.0);
    
    draw.to_frame(app, &frame).unwrap();
}
