use std::iter::Sum;

use nannou::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    re: f32,
    im: f32,
}

impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Complex::zero(), |acc, x| acc + x)
    }
}

impl From<Vec2> for Complex {
    fn from(vec: Vec2) -> Self {
        Self {
            re: vec.x,
            im: vec.y,
        }
    }
}

impl Into<Vec2> for Complex {
    fn into(self) -> Vec2 {
        Vec2::new(self.re, self.im)
    }
}

impl Into<Vec2> for &Complex {
    fn into(self) -> Vec2 {
        Vec2::new(self.re, self.im)
    }
}   

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl std::ops::Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }
}

impl std::ops::Mul<u32> for Complex {
    type Output = Self;

    fn mul(self, scalar: u32) -> Self {
        Self {
            re: self.re * scalar as f32,
            im: self.im * scalar as f32,
        }
    }
}

impl std::ops::Div<f32> for Complex {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            re: self.re / scalar,
            im: self.im / scalar,
        }
    }
}

impl std::ops::Div<u32> for Complex {
    type Output = Self;

    fn div(self, scalar: u32) -> Self {
        Self {
            re: self.re / scalar as f32,
            im: self.im / scalar as f32,
        }
    }
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn re(&self) -> f32 {
        self.re
    }

    pub fn im(&self) -> f32 {
        self.im
    }

    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn argument(&self) -> f32 {
        self.im.atan2(self.re)
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn i() -> Self {
        Self { re: 0.0, im: 1.0 }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn exp(&self) -> Self {
        let magnitude = self.magnitude();
        let argument = self.argument();
        Self {
            re: magnitude * argument.cos(),
            im: magnitude * argument.sin(),
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

pub fn load_from_file(file: &str) -> Result<Vec<Complex>, String> {
    let content = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
    let mut chars = content.chars();
    let mut complexes = Vec::new();
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        }

        if c == '(' {
            let mut re_buf = String::new();
            let mut im_buf = String::new();
            let mut is_imaginary = false;

            while let Some(c) = chars.next() {
                match c {
                    ' ' | '\n' | '\t' => continue,
                    ')' => {
                        if let (Ok(re), Ok(im)) = (re_buf.parse::<f32>(), im_buf.parse::<f32>()) {
                            complexes.push(Complex::new(re, im));
                        } else {
                            return Err(format!(
                                "Failed to parse complex number from '{}' and '{}'",
                                re_buf, im_buf
                            ));
                        }
                        break;
                    }
                    ',' => is_imaginary = true,
                    _ if c.is_digit(10) || c == '-' || c == '.' => {
                        if is_imaginary {
                            im_buf.push(c);
                        } else {
                            re_buf.push(c);
                        }
                    }
                    _ => return Err(format!("Unexpected character '{}' in complex number", c)),
                }
            }
        }
    }
    let max_dist = complexes.iter().map(|c| c.magnitude()).fold(0.0, f32::max);
    complexes = complexes.iter_mut().map(|c| *c / max_dist * 400).collect();
    Ok(complexes)
}