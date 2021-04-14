extern crate clap;
extern crate lodepng;
extern crate nalgebra;
extern crate nalgebra_glm;
extern crate num_integer;
extern crate rand;

mod args;
mod camera;
mod intersect;
mod material;
mod output;
mod radiance;
mod rays;
mod render;
mod time;
mod triangle;

use crate::args::{parse_arguments, Arguments};
use crate::camera::create_camera;
use crate::output::output;
use crate::rays::create_rays;
use crate::render::render;
use crate::time::Time;
use crate::triangle::create_triangles;

fn run(args: &Arguments) {
    let mut time = Time::new();

    let camera = create_camera(args);
    let camera_time = time.record();

    let rays = create_rays(args, &camera.step, &camera.viewport);
    let rays_time = time.record();

    let triangles = create_triangles();
    let triangles_time = time.record();

    let pixels = render(args, &camera, &triangles, rays);
    let pixels_time = time.record();

    output(pixels.as_slice(), args.width.into(), args.height.into());
    let output_time = time.record();
    let total_time = time.elapsed();

    println!("Camera created in {} us", camera_time.as_micros());
    println!("Rays created in {} us", rays_time.as_micros());
    println!("Triangles created in {} us", triangles_time.as_micros());
    println!("Render created in {} us", pixels_time.as_micros());
    println!("Output created in {} us", output_time.as_micros());
    println!("Finished in {} us", total_time.as_micros());
}

fn main() {
    run(&parse_arguments());
}
