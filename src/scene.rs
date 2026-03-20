use crate::core::Color;
use crate::transform::Transform;

pub struct ObjectData {
    pub pose: Transform,
    pub color: Color,
}

pub enum Object {
    Sphere(ObjectData, f64),
    Cube(ObjectData, (f64, f64, f64)),
}

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {}
