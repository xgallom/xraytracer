use crate::camera::Camera;
use crate::rays::position_along;
use crate::triangle::TriangleLike;
use crate::triangle::{Triangle, Triangles};
use nalgebra_glm::Vec3;

#[derive(Debug)]
pub struct Nearest {
    pub(crate) id: usize,
    pub(crate) distance: f32,
    pub(crate) back_facing: bool,
    pub(crate) position: Vec3,
    pub(crate) normal: Vec3,
}

struct TriangleIntersect {
    id: usize,
    distance: f32,
    det: f32,
    u: f32,
    v: f32,
}

const EPSILON: f32 = 0.000000001f32;

fn intersect_triangle(
    camera: &Camera,
    triangles: &[Triangle],
    normal_ray: &Vec3,
) -> Option<TriangleIntersect> {
    let nearest = triangles.iter().enumerate().fold(
        TriangleIntersect {
            id: usize::MAX,
            distance: f32::MAX,
            det: 0f32,
            u: 0f32,
            v: 0f32,
        },
        |nearest, (id, triangle)| {
            let p = normal_ray.cross(&triangle.v());
            let det: f32 = triangle.u().dot(&p);

            if det.abs() >= EPSILON {
                let inv_det = det.recip();
                let t: Vec3 = &camera.origin - triangle.column(0);
                let u = t.dot(&p) * inv_det;
                let q = t.cross(&triangle.u());
                let v = normal_ray.dot(&q) * inv_det;

                if !((u < 0f32) | (u > 1f32) | (v < 0f32) | (u + v > 1f32)) {
                    let distance: f32 = triangle.v().dot(&q) * inv_det;

                    if distance > EPSILON && distance < nearest.distance {
                        return TriangleIntersect {
                            id,
                            distance,
                            det,
                            u,
                            v,
                        };
                    }
                }
            }

            nearest
        },
    );

    if nearest.id == usize::MAX {
        None
    } else {
        Some(nearest)
    }
}

fn nearest_from_intersect(
    camera: &Camera,
    triangle_normals: &[Triangle],
    intersect: TriangleIntersect,
    normal_ray: &Vec3,
) -> Nearest {
    let triangle_normal = &triangle_normals[intersect.id];

    let normal: Vec3 = (triangle_normal.u().scale(intersect.u)
        + triangle_normal.v().scale(intersect.v)
        + triangle_normal.column(0))
    .normalize();

    let back_facing = intersect.det < EPSILON;

    Nearest {
        id: intersect.id,
        distance: intersect.distance,
        back_facing,
        position: position_along(&camera.origin, normal_ray, intersect.distance),
        normal: if back_facing { -normal } else { normal },
    }
}

pub fn intersect_triangles(camera: &Camera, triangles: &Triangles, ray: &Vec3) -> Option<Nearest> {
    let normal_ray = &ray.normalize();

    intersect_triangle(camera, triangles.vertices.as_slice(), normal_ray).map(|intersect| {
        nearest_from_intersect(camera, triangles.normals.as_slice(), intersect, normal_ray)
    })
}
