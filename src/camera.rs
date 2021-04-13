use crate::args::Arguments;
use nalgebra_glm::{vec3, Vec3};
use std::ops::Div;

pub struct Camera {
    pub(crate) step: Vec3,
    pub(crate) viewport: Vec3,
    pub(crate) origin: Vec3,
}

pub fn create_camera(args: &Arguments) -> Camera {
    let cols = f32::from(args.width - 1);
    let rows = f32::from(args.height - 1);

    let viewport_origin = vec3(0f32, 0f32, args.viewport_distance);
    let viewport_half_dim_x = args.viewport_distance * args.fov.to_radians().div(2f32).tan();
    let viewport_half_dim = vec3(viewport_half_dim_x, viewport_half_dim_x * cols / rows, 0f32);

    let step = viewport_half_dim
        .scale(2f32)
        .component_div(&vec3(cols, rows, 1f32));
    let viewport = viewport_origin - viewport_half_dim;
    let origin = vec3(0f32, 0f32, args.camera_distance);

    Camera {
        step,
        viewport,
        origin,
    }
}
