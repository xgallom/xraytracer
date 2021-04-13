use crate::args::Arguments;
use crate::camera::Camera;
use crate::intersect::intersect_triangles;
use crate::triangle::Triangles;
use nalgebra_glm::Vec3;
use num_integer::Integer;

pub fn compute_radiance(
    args: &Arguments,
    camera: &Camera,
    triangles: &Triangles,
    ray: &Vec3,
    depth: u32,
) -> lodepng::RGB<u8> {
    if depth >= args.max_depth {
        return lodepng::RGB::new(0x00u8, 0x00u8, 0x00u8);
    }

    // let u_samples_count = if depth == 0 {
    //     args.u_samples_first_bounce
    // } else {
    //     1u32
    // };

    // let v_samples_count = if depth == 0 {
    //     args.v_samples_first_bounce
    // } else {
    //     1u32
    // };

    match intersect_triangles(camera, triangles, ray) {
        Some(intersection) => {
            // TODO: Implement materials
            match intersection.id.mod_floor(&12usize) {
                0 | 1 => lodepng::RGB::new(0xffu8, 0x00u8, 0xffu8),
                2 | 3 => lodepng::RGB::new(0xffu8, 0x00u8, 0xffu8),
                4 | 5 => lodepng::RGB::new(0x00u8, 0xffu8, 0xffu8),
                6 | 7 => lodepng::RGB::new(0x00u8, 0xffu8, 0xffu8),
                8 | 9 => lodepng::RGB::new(0xffu8, 0xffu8, 0x00u8),
                10 | 11 => lodepng::RGB::new(0xffu8, 0xffu8, 0x00u8),
                _ => lodepng::RGB::new(0xffu8, 0xffu8, 0xffu8),
            }
        }
        None => lodepng::RGB::new(0x33u8, 0x33u8, 0x33u8),
    }
}
