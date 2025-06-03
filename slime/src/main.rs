use nannou::{color, prelude::*};
mod slime;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug)]
struct Model {
    slimes: Vec<slime::Slime>,
}

fn model(_app: &App) -> Model {
    Model {
        slimes: vec![slime::Slime::blob(
            slime::SlimeProps {
                cohesion: 2.0,
                min_cohesion_distance: 30.0,
                pressure: 1.0,
                color: nannou::color::Rgba::new(0.1, 0.8, 0.25, 1.0),

                split_threshold: 100.0,
                split_chance: 0.0,
                split_force: 0.0,

                merge_threshold: 100.0,
                merge_chance: 0.0,
                merge_force: 0.0,

                vision_radius: 0.0,

                velocity_decay: 0.8,
            },
            30,
            100.0,
            0.0,
            0.0,
        )],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let slimes = model.slimes.clone();
    for (_, slime) in model.slimes.iter_mut().enumerate() {
        slime.update(0.6, &slimes, app.mouse.position());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.slimes.iter().for_each(|slime| {
        slime.draw(&draw);
        for (i, point) in slime.body.iter().enumerate() {
            draw.arrow()
                .start(point.position)
                .end(point.position + point.velocity)
                .color(rgba(1.0, 1.0, 1.0, 0.5))
                .weight(1.0);

            draw.arrow()
                .start(point.position)
                .end(point.position + slime._get_tension(i))
                .color(rgba(0.0, 0.0, 1.0, 0.5))
                .weight(2.0);

            draw.arrow()
                .start(point.position)
                .end(point.position + slime._get_pressure(i))
                .color(rgba(1.0, 0.0, 0.0, 0.5))
                .weight(2.0);

            draw.arrow()
                .start(point.position)
                .end(point.position + slime._get_target_force(i, app.mouse.position()))
                .color(rgba(0.0, 0.1, 0.0, 0.5))
                .weight(2.0);
        }
        draw.ellipse()
            .xy(slime.centroid())
            .radius(slime.core_radius())
            .color(rgba(1.0, 0.0, 0.0, 0.5));
    });

    draw.ellipse()
        .xy(app.mouse.position())
        .radius(8.0)
        .color(rgba(1.0, 1.0, 1.0, 0.4));

    draw.to_frame(app, &frame).unwrap();
}
