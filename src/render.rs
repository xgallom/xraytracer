use crate::args::Arguments;
use crate::camera::Camera;
use crate::radiance::compute_radiance;
use crate::rays::Rays;
use crate::triangle::Triangles;
use nalgebra_glm::Vec3;

const BYTES_PER_PIXEL: usize = 3;

pub fn render(args: &Arguments, camera: &Camera, triangles: &Triangles, rays: Rays) -> Vec<u8> {
    let mut pixels: Vec<u8> = Vec::with_capacity(rays.ncols() * BYTES_PER_PIXEL);

    rays.column_iter().for_each(|ray| {
        let radiance = compute_radiance(args, &camera, &triangles, &ray.into(), 0);

        for &value in radiance.iter() {
            pixels.push((255f32 * value) as u8);
        }
    });

    pixels
}
