use crate::args::Arguments;
use crate::camera::Camera;
use crate::intersect::intersect_triangles;
use crate::material::Material;
use crate::triangle::Triangles;
use nalgebra_glm::{vec3, Vec3};
use num_integer::Integer;

// For details, see:
// http://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
fn compute_reflectance(normal: &Vec3, incoming: &Vec3, (ior_from, ior_to): (f32, f32)) -> f32 {
    let ior_ratio = ior_from / ior_to;
    let cos_theta_i = -normal.dot(incoming);
    let sin_theta_t_squared = ior_ratio * ior_ratio * (1f32 - cos_theta_i * cos_theta_i);

    if sin_theta_t_squared > 1f32 {
        1.0
    } else {
        let cos_theta_t = (1f32 - sin_theta_t_squared).sqrt();

        let r_perpendicular = (ior_from * cos_theta_i - ior_to * cos_theta_t)
            / (ior_from * cos_theta_i + ior_to * cos_theta_t);
        let r_parallel = (ior_from * cos_theta_i - ior_to * cos_theta_t)
            / (ior_from * cos_theta_i + ior_to * cos_theta_t);

        r_perpendicular * r_perpendicular + r_parallel * r_parallel
    }
}

pub fn compute_radiance(
    args: &Arguments,
    camera: &Camera,
    triangles: &Triangles,
    ray: &Vec3,
    depth: u32,
) -> Vec3 {
    if depth >= args.max_depth {
        return vec3(0f32, 0f32, 0f32);
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

    let normal_ray = &ray.normalize();

    let material = Material::make_diffuse(vec3(0f32, 0.7f32, 0.85f32));
    intersect_triangles(camera, triangles, ray, normal_ray)
        .map(|intersection| {
            if intersection.distance < 0.1 {
                return vec3(1f32, 1f32, 1f32);
            }
            if args.render_preview {
                return material.diffuse;
            }

            let ior = if intersection.back_facing {
                (material.index_of_refraction, 1f32)
            } else {
                (1f32, material.index_of_refraction)
            };

            let reflectivity = if material.reflectivity.is_sign_negative() {
                compute_reflectance(&intersection.normal, normal_ray, ior)
            } else {
                material.reflectivity
            };

            material.diffuse
        })
        .unwrap_or(vec3(0f32, 0f32, 0f32))
}
