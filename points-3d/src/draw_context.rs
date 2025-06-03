use nannou::prelude::*;
use crate::matrix::Matrix4;

fn make_proj_matrix(
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> Matrix4 {
    let f = 1.0 / (fov.to_radians() / 2.0).tan();
    let range = 1.0 / (near - far);

    Matrix4::new(
        [f / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (near + far) * range, -1.0],
        [0.0, 0.0, near * far * range * 2.0, 0.0],
    )
}

pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Default::default(),
            rotation: Default::default(),
            fov: 45.0,
            aspect_ratio: 1.0,
            near: 0.1,
            far: 100.0,
        }
    }
}

pub struct DrawContext {
    camera: Camera,
}

impl Default for DrawContext {
    fn default() -> Self {
        DrawContext {
            camera: Default::default()
        }
    }
}

impl DrawContext {
    fn new(camera: Camera) -> Self {
        DrawContext { camera }
    }
}