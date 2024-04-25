extern crate nannou;
use nannou::prelude::*;
mod model;

macro_rules! BG_COLOR {
    () => {
        rgb(0.05, 0.05, 0.05)
    };
}

fn main() {
    nannou::app(model::model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}



fn update(_app: &App, _model: &mut model::Model, _update: Update) {
    _model.update_count += 1;
    if let Some((_, time)) = _model.last_click {
        if time.elapsed() > std::time::Duration::from_secs(3) {
            _model.last_click = None;
        } else if _app.mouse.buttons.left().is_down() {
            _model.last_click = Some((pt2(_app.mouse.x, _app.mouse.y), std::time::Instant::now()));
        }
    }
}

fn event(_app: &App, _model: &mut model::Model, _event: Event) {
    match _event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            KeyPressed(Key::Escape) => {
                _app.quit();
            }
            MousePressed(MouseButton::Left) => {
                // println!("Mouse left button pressed at ({},{})", _app.mouse.x, _app.mouse.y);
                _model.last_click =
                    Some((pt2(_app.mouse.x, _app.mouse.y), std::time::Instant::now()));
            }
            Resized(_size) => {
                _model.last_click = None;
            }
            _ => (),
        },
        _ => (),
    }
}

fn view(_app: &App, _model: &model::Model, frame: Frame) {
    let draw = _app.draw();
    let bounds = frame.rect();

    draw.background().color(BG_COLOR!());

    let elapsed_time = _model.update_count as f32 / 60.0;

    draw.rect()
        .wh(bounds.wh())
        .stroke_color(rgb(100 as u8, 100 as u8, 200 as u8))
        .stroke_weight(3.0)
        .no_fill();

    draw.text(&format!("Elapsed time: {:.2}s", elapsed_time))
        .xy(bounds.xy())
        .wh(bounds.pad(10.0).wh())
        .font_size(16)
        .color(WHITE)
        .left_justify()
        .align_text_top();

    draw.arrow()
        .start(pt2(0.0, 0.0))
        .end(pt2(100.0, 0.0))
        .weight(2.0)
        .start_cap_round()
        .color(WHITE);

    draw.text("(100.0, 0.0)")
        .x_y(100.0, -20.0)
        .font_size(12)
        .color(WHITE);

    draw.arrow()
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, 100.0))
        .weight(2.0)
        .start_cap_round()
        .color(WHITE);

    draw.text("(0.0, 100.0)")
        .roll(PI / 2.0)
        .x_y(-20.0, 100.0)
        .font_size(12)
        .color(WHITE);

    for i in 1..9 {
        let x = i as f32 * 10.0;
        let y = i as f32 * 10.0;
        draw.line()
            .start(pt2(0.0, y))
            .end(pt2(90.0, y))
            .weight(1.0)
            .caps_round()
            .color(GRAY);
        draw.line()
            .start(pt2(x, 0.0))
            .end(pt2(x, 90.0))
            .weight(1.0)
            .caps_round()
            .color(GRAY);
    }

    if let Some((last_click, _)) = _model.last_click {
        draw.ellipse()
            .xy(last_click)
            .wh(vec2(10.0, 10.0))
            .color(RED);

        draw.ellipse()
            .xy(last_click)
            .wh(vec2(10.0, 10.0) * 2.0)
            .stroke_color(RED)
            .stroke_weight(2.0)
            .no_fill();

        draw.line()
            .start(last_click + vec2(10.0, 0.0))
            .end(last_click + vec2(-10.0, 0.0))
            .weight(2.0)
            .color(RED);

        draw.line()
            .start(last_click + vec2(0.0, 10.0))
            .end(last_click + vec2(0.0, -10.0))
            .weight(2.0)
            .color(RED);
    }

    draw.to_frame(_app, &frame).unwrap()
}
