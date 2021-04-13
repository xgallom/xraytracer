use crate::args::Arguments;
use nalgebra::Matrix3xX;
use nalgebra_glm::Vec3;
// use rand::distributions::Distribution;
// use rand::thread_rng;

pub type Rays = Matrix3xX<f32>;

pub fn position_along(origin: &Vec3, ray: &Vec3, distance: f32) -> Vec3 {
    origin + ray.scale(distance)
}

pub fn create_rays(args: &Arguments, step: &Vec3, viewport: &Vec3) -> Rays {
    // let unit = rand::distributions::Uniform::new(-0.5f32, 0.5f32);
    // let mut rng = thread_rng();

    let height = args.height as usize;
    let width = args.width as usize;

    Rays::from_fn(height * width, |d, n| -> f32 {
        // TODO: Implement ray rng
        match d {
            0 => viewport.x + step.x * ((n % width) as f32/*+ unit.sample(&mut rng)*/),
            1 => viewport.y + step.y * ((n / width) as f32/*+ unit.sample(&mut rng)*/),
            _ => args.viewport_distance,
        }
    })
}
