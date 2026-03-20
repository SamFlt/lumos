use crate::{camera::Camera, transform::Transform};

pub mod camera;
pub mod transform;

fn main() {
    let x = Camera {
        pose: Transform::new(),
        focal_length: 0.032,
        sensor_width: 0.05,
        sensor_height: 0.05,
        width_resolution: 1200,
        height_resolution: 800,
    };
    println!("Hello, world!");
}
