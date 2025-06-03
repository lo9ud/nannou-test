use nannou::{color::{rgb, Rgba8}, event::WindowEvent::Touch, glam::{vec3, vec4, Vec2}, Draw};

pub struct Style {
    pub arm: ArmStyle,
    pub hinge: HingeStyle,
    pub tip: TipStyle,
}

pub enum ArmStyle {
    NoDraw,
    Static(Rgba8),
    Linear(Rgba8, Rgba8)
}

pub enum HingeStyle {
    None,
    Cicle(f32)
}

pub enum TipStyle {
    None,
    Circle(f32)
}

impl Style {
     pub fn draw(&self, draw: &Draw, from: Vec2, to: Vec2) {
        match self.arm {
            ArmStyle::NoDraw => {},
            ArmStyle::Static(c) => {draw.line().start(from).end(to).color(c);}
            ArmStyle::Linear(a, b) => {
                const SEGMENTS: usize = 30;
                let delta = (to - from) / SEGMENTS as f32;
                let a_c = vec4(a.red as f32, a.blue as f32, a.green as f32, a.alpha as f32);
                let b_c = vec4(b.red as f32, b.blue as f32, b.green as f32, b.alpha as f32);
                let c_delta = (b_c-a_c)/SEGMENTS as f32;
                for i in 0..SEGMENTS {
                    let i = i as f32;
                    let c_vec = a_c + c_delta*i;
                    let c = rgb(c_vec.x as u8, c_vec.y as u8, c_vec.z as u8);
                    draw.line()
                        .start(from + delta * i)
                        .end(from + delta * (i+1.0))
                        .color(c);
                }
            }
        };

        match self.hinge {
            HingeStyle::None => {},
            HingeStyle::Cicle(r) => {
                draw.ellipse()
                    .xy(from)
                    .radius(r);
            }
        };

        match self.tip {
            TipStyle::None => {},
            TipStyle::Circle(r) => {
                draw.ellipse()
                    .xy(to)
                    .radius(r);
            }
        };
    }
}