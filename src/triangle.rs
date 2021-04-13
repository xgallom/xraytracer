use nalgebra_glm::{vec3, Mat3x3, Vec3};
use num_integer::Integer;
use std::ops::AddAssign;

pub type Triangle = Mat3x3;

pub trait TriangleLike {
    fn u(&self) -> Vec3;
    fn v(&self) -> Vec3;
    fn face_normal(&self) -> Vec3;
}

impl TriangleLike for Triangle {
    fn u(&self) -> Vec3 {
        self.column(1) - self.column(0)
    }

    fn v(&self) -> Vec3 {
        self.column(2) - self.column(0)
    }

    fn face_normal(&self) -> Vec3 {
        self.u().cross(&self.v()).normalize()
    }
}

pub struct Triangles {
    pub(crate) vertices: Vec<Triangle>,
    pub(crate) normals: Vec<Triangle>,
}

pub fn create_triangles() -> Triangles {
    let mut vertices_data: Vec<Triangle> = vec![
        Triangle::new(0f32, 1f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32),
        Triangle::new(1f32, 1f32, 0f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32),
        Triangle::new(0f32, 1f32, 0f32, 0f32, 0f32, 1f32, 1f32, 1f32, 1f32),
        Triangle::new(1f32, 1f32, 0f32, 1f32, 0f32, 1f32, 1f32, 1f32, 1f32),
        Triangle::new(0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32),
        Triangle::new(1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32),
        Triangle::new(0f32, 0f32, 1f32, 1f32, 1f32, 1f32, 0f32, 1f32, 0f32),
        Triangle::new(1f32, 0f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 0f32),
        Triangle::new(0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32),
        Triangle::new(0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 1f32, 0f32, 1f32),
        Triangle::new(1f32, 1f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32),
        Triangle::new(1f32, 1f32, 1f32, 1f32, 1f32, 0f32, 1f32, 0f32, 1f32),
    ];

    vertices_data.iter_mut().for_each(|triangle| {
        triangle.column_iter_mut().for_each(|mut vertex| {
            vertex.add_assign(&vec3(-3.5f32, -3.5f32, 0f32));
        })
    });

    let mut vertices: Vec<Triangle> = Vec::with_capacity(12 * 4);

    for n in 0..16 {
        let mut cloned_vertices = vertices_data.clone();
        cloned_vertices.iter_mut().for_each(|triangle| {
            triangle.column_iter_mut().for_each(|mut vertex| {
                let (h, w) = n.div_mod_floor(&4);
                vertex.add_assign(&vec3(2f32 * w as f32, 2f32 * h as f32, 0f32));
            });
        });

        vertices.append(&mut cloned_vertices);
    }

    let normals: Vec<Triangle> = vertices
        .iter()
        .map(|triangle| {
            let face_normal = triangle.face_normal();
            Triangle::from_fn(|x, _| face_normal[x])
        })
        .collect();

    Triangles { vertices, normals }
}
