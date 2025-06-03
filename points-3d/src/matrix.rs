use std::ops::{Deref, DerefMut};

pub struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4 {
            data: [
                [1.0, 0.0, 0.0, 0.0], //
                [0.0, 1.0, 0.0, 0.0], //
                [0.0, 0.0, 1.0, 0.0], //
                [0.0, 0.0, 0.0, 1.0], //
            ],
        }
    }
}

impl std::ops::Mul<nannou::prelude::Vec3> for Matrix4 {
    type Output = nannou::prelude::Vec3;

    fn mul(self, rhs: nannou::prelude::Vec3) -> Self::Output {
        let m = &self.data;
        let x = m[0][0] * rhs.x + m[0][1] * rhs.y + m[0][2] * rhs.z + m[0][3];
        let y = m[1][0] * rhs.x + m[1][1] * rhs.y + m[1][2] * rhs.z + m[1][3];
        let z = m[2][0] * rhs.x + m[2][1] * rhs.y + m[2][2] * rhs.z + m[2][3];
        nannou::prelude::Vec3::new(x, y, z)
    }
}

impl Deref for Matrix4 {
    type Target = [[f32; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Matrix4 {
    pub fn new(
        row1: [f32; 4],
        row2: [f32; 4],
        row3: [f32; 4],
        row4: [f32; 4],
    ) -> Self {
        Matrix4 {
            data: [row1, row2, row3, row4],
        }
    }
}
