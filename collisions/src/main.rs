mod ball;

use nannou::prelude::*;

const ELASTICITY: f32 = 1.0;
const FRICTION: f32 = 0.01;

enum Drag {
    None,
    Getting,
    Dragging(Point2),
}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

struct Model {
    _window: window::Id,
    balls: Vec<ball::Ball>,
    drag_init: Drag,
    drag_end: Drag,
    gravity: Drag,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).build().unwrap();
    let mut balls = Vec::new();
    // (center, radius, count, colour)
    let clouds: Vec<(Point2, f32, u32, Rgb<u8>)> = vec![
        (Point2::new(0.0, 0.0), 5.0, 8, rgb(255, 0, 0)),
        (Point2::new(-200.0, -200.0), 5.0, 7, rgb(0, 255, 0)),
        (Point2::new(200.0, -200.0), 5.0, 7, rgb(0, 0, 255)),
        (Point2::new(-200.0, 200.0), 5.0, 7, rgb(255, 255, 0)),
        (Point2::new(200.0, 200.0), 5.0, 7, rgb(255, 165, 0)),
    ];

    for cloud_spec in clouds {
        let (xy, radius, count, colour) = cloud_spec;
        balls.push(ball::Ball::new(xy, Vec2::ZERO, radius, colour));
        for i in 0..(count * count) {
            let x = map_range(
                i % count,
                0,
                count,
                -radius * count as f32,
                radius * count as f32,
            ) + random_range(-radius, radius);
            let y = map_range(
                i / count,
                0,
                count,
                -radius * count as f32,
                radius * count as f32,
            ) + random_range(-radius, radius);
            let p = xy + Vec2::new(x, y);
            let v = (p - xy).normalize() * random_range(0.75, 1.25);
            let b = ball::Ball::new(p, v, radius, colour);
            balls.push(b);
        }
    }

    Model {
        _window,
        balls,
        drag_init: Drag::None,
        drag_end: Drag::None,
        gravity: Drag::None,
    }
}

fn collide(a: &mut ball::Ball, b: &mut ball::Ball) {
    let vcom = (a.velocity * a.mass() + b.velocity * b.mass()) / (a.mass() + b.mass());

    let a_v = (a.mass() - b.mass()) / (a.mass() + b.mass()) * a.velocity
        + (2.0 * b.mass()) / (a.mass() + b.mass()) * b.velocity;
    let b_v = (b.mass() - a.mass()) / (a.mass() + b.mass()) * b.velocity
        + (2.0 * a.mass()) / (a.mass() + b.mass()) * a.velocity;

    // a.velocity = a_v;
    // b.velocity = b_v;

    a.velocity = (1.0 + ELASTICITY) * vcom - ELASTICITY * a.velocity;
    b.velocity = (1.0 + ELASTICITY) * vcom - ELASTICITY * b.velocity;

    let overlap = a.radius + b.radius - a.position.distance(b.position) + 2.0;
    let a_move = overlap * (b.position - a.position).normalize() * 0.5;
    let b_move = overlap * (a.position - b.position).normalize() * 0.5;
    a.position -= a_move;
    b.position -= b_move;
}

fn bounce_wall(ball: &mut ball::Ball, bounds: &Rect) {
    if ball.position.x - ball.radius < bounds.left() {
        ball.position.x = bounds.left() + ball.radius;
        ball.velocity.x *= -1.0;
    }
    if ball.position.x + ball.radius > bounds.right() {
        ball.position.x = bounds.right() - ball.radius;
        ball.velocity.x *= -1.0;
    }
    if ball.position.y - ball.radius < bounds.bottom() {
        ball.position.y = bounds.bottom() + ball.radius;
        ball.velocity.y *= -1.0;
    }
    if ball.position.y + ball.radius > bounds.top() {
        ball.position.y = bounds.top() - ball.radius;
        ball.velocity.y *= -1.0;
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    for ball in &mut model.balls {
        bounce_wall(ball, &bounds);
    }
    // get mutable pairs of balls that are colliding and apply collide to them
    for i in 0..model.balls.len() {
        for j in (i + 1)..model.balls.len() {
            let (a_chunk, b_chunk) = model.balls.split_at_mut(j);
            let a = &mut a_chunk[i];
            let b = &mut b_chunk[0];
            if a.position.distance(b.position) < (a.radius + b.radius) {
                collide(a, b);
            }
        }
    }

    // apply gravity to all balls
    if let Drag::Dragging(g) = model.gravity {
        for ball in &mut model.balls {
            ball.velocity += (g - ball.position).normalize();
        }
    }

    // apply friction to all balls
    for ball in &mut model.balls {
        ball.velocity *= 1.0 - FRICTION;
    }

    for ball in &mut model.balls {
        ball.position += ball.velocity;
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(simple),
        } => match simple {
            WindowEvent::MousePressed(MouseButton::Left) => {
                model.drag_init = Drag::Getting;
                model.drag_end = Drag::Getting;
            }
            WindowEvent::MousePressed(MouseButton::Right) => {
                model.gravity = Drag::Getting;
            }
            WindowEvent::MouseReleased(MouseButton::Left) => {
                if let (Drag::Dragging(start), Drag::Dragging(end)) =
                    (&model.drag_init, &model.drag_end)
                {
                    let v = (*start - *end) / 50.0;
                    let p = *end;
                    let radius = 15.0;
                    let colour = rgb(255, 255, 255);
                    model.balls.push(ball::Ball::new(p, v, radius, colour));
                }
                model.drag_end = Drag::None;
                model.drag_init = Drag::None;
            }
            WindowEvent::MouseReleased(MouseButton::Right) => {
                model.gravity = Drag::None;
            }
            WindowEvent::MouseMoved(pos) => {
                if let Drag::Getting = model.drag_init {
                    model.drag_init = Drag::Dragging(pos);
                }
                model.drag_end = Drag::Dragging(pos);
                if let Drag::Getting = model.gravity {
                    model.gravity = Drag::Dragging(pos);
                }
                if let Drag::Dragging(_) = model.gravity {
                    model.gravity = Drag::Dragging(pos);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    if let (Drag::Dragging(start), Drag::Dragging(end)) = (&model.drag_init, &model.drag_end) {
        draw.arrow()
            .start(*end)
            .end(*start)
            .weight(start.distance(*end) / 50.0)
            .color(RED);
    }
    for ball in &model.balls {
        ball.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
