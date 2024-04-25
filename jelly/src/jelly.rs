use nannou::prelude::*;

const PULSE_AGGRESSION: u8 = 7;
fn raw_pulse(t: f32) -> f32 {
    -(t) * (t - 1.0) * (t + 1.0).powi(PULSE_AGGRESSION as i32)
}

pub fn pulse(t: f32) -> f32 {
    let t = t % 1.0;
    let a = PULSE_AGGRESSION as f32;
    let ajust = 0.5 * (a.powi(2) + 2.0 * a + 9.0).sqrt() / (a + 2.0) + 0.5 * (a - 1.0) / (a + 2.0);
    raw_pulse(t) / raw_pulse(ajust)
}

pub struct DrawSettings {
    pub color: Rgba,
    pub size: f32,
    pub rotation: f32,
    pub scale: f32,
    pub alpha: f32,
}

impl Default for DrawSettings {
    fn default() -> Self {
        DrawSettings {
            color: rgba(1.0, 1.0, 1.0, 1.0),
            size: 10.0,
            rotation: 0.0,
            scale: 1.0,
            alpha: 1.0,
        }
    }
}
pub struct JellyMovement {
    pub phase: f32,
    pub period: f32,
    pub phase_fn: fn(f32) -> f32,
}

impl JellyMovement {
    pub fn update(&mut self) {
        self.phase = ((self.phase + 0.01) % self.period)/self.period;
    }
    pub fn impetus(&mut self) -> f32 {
        (self.phase_fn)(self.phase)
    }
}

impl Default for JellyMovement {
    fn default() -> Self {
        JellyMovement {
            phase: 0.0,
            period: 1.0,
            phase_fn: pulse,
        }
    }
}
pub struct Jelly {
    pub position: Point2,
    pub velocity: Vec2,
    pub direction: f32,
    pub size: f32,
    pub movement: JellyMovement,
    pub draw_settings: DrawSettings,
}

impl Jelly {
    pub fn update(&mut self) {
        self.movement.update();
        let impetus = self.movement.impetus()*vec2(1.0,0.0).rotate(self.direction);
        let friction = self.velocity * -0.01;
        self.velocity += (
            impetus
            + friction
        )/self.size;
        self.position += self.velocity;
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .color(self.draw_settings.color)
            .w_h(self.draw_settings.size*2.0, self.draw_settings.size)
            .rotate(self.direction)
            .rgba(1.0, 1.0, 1.0, self.draw_settings.alpha);
    }
}

impl Default for Jelly {
    fn default() -> Self {
        Jelly {
            position: vec2(0.0, 0.0),
            velocity: vec2(1.0, 0.0).rotate(random_range(0.0, PI * 2.0)),
            direction: 0.0,
            size: 1.0,
            movement: JellyMovement::default(),
            draw_settings: DrawSettings::default(),
        }
    }
}
