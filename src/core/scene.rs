use ndarray::{Axis, s};
use ndarray_linalg::{Norm, NormalizeAxis};

use crate::core::camera::{RayResult, RayResults, Rays};
use crate::core::color::Color;
use crate::core::math::{self, QuadraticSolutions};
use crate::core::transform::{Transform, Vec3};

pub struct ObjectData {
    pub pose: Transform,
    pub color: Color,
}

impl ObjectData {
    pub fn transform(self: &Self, t: &Transform) -> Self {
        ObjectData {
            pose: &self.pose * t,
            color: self.color
        }
    }
}



pub enum Object {
    Sphere(ObjectData, f64),
    Plane(ObjectData),
    Cube(ObjectData, (f64, f64, f64)),
}


impl Object {
    pub fn color(self: &Self) -> Color {
        match self {
            Object::Sphere(d, _) | Object::Plane(d) | Object::Cube(d, _) => d.color
        }
    }
    pub fn sphere(pose: Transform, radius: f64) -> Object {
        Object::Sphere(ObjectData { pose: pose, color: Color {r: 1.0, g: 0.0, b: 0.0} }, radius)
    }
    pub fn intersect(self: &Self, rays: &Rays) -> Vec<RayResult> {
        match self {
            Object::Sphere(object_data, radius) => {

                let center = object_data.pose.position().to_ndarray();
                let solutions = rays.origins.axis_iter(Axis(0)).zip(rays.directions.axis_iter(Axis(0))).map(|(o, d)|{
                    let l = &o - &center;
                    let a = 1.0;
                    
                    let b = ((&d * 2.0) * &l).sum();
                    let c = (l.pow2()).sum() - (radius * radius);
                    let disc = b * b - 4.0 * a * c;
                    match  disc {
                        ..0.0 => QuadraticSolutions::None,
                        0.0 => {
                            let s = -b / (2.0 * a);
                            if s >= 0.0 {
                                QuadraticSolutions::One(s)
                            }  else {
                                QuadraticSolutions::None
                            }
                        },
                        0.0.. => {
                            let sqdisc = disc.sqrt();
                            let sol1 = (-b + sqdisc) / 2.0 * a;
                            let sol2 = (-b - sqdisc) / (2.0 * a);

                            if sol1 < 0.0 && sol2 < 0.0 {
                                QuadraticSolutions::None
                            } else {
                                if sol1 > sol2 {
                                    QuadraticSolutions::Two(sol2, sol1)
                                } else {
                                    QuadraticSolutions::Two(sol1, sol2)
                                }
                            }

                        },
                        _ => panic!("This case should not happen")
                    }
                });

                solutions.enumerate().filter(|s| match s.1 {
                    QuadraticSolutions::None => false,
                    _ => true
                }).map(|(id, solution)| {
                    
                    let t = match solution {
                        QuadraticSolutions::One(s) => s,
                        QuadraticSolutions::Two(s, _) => s,
                        _ => panic!("Cases with no solutions should have been filtered")
                    };
                    let hit_point = &rays.origins.slice(s![id, ..]) + &rays.directions.slice(s![id, ..]) * t; 
                    let mut normal = &hit_point - &center;
                    let norm = normal.norm_l2();
                    normal /= norm;
                    RayResult {
                        id: id,
                        hit_point: hit_point,
                        dist: t,
                        normal: normal,
                    }
                }).collect()
            },
            Object::Plane(data) => {

                let n = data.pose.forward().to_ndarray();
                let d = (data.pose.position().to_ndarray() * &n).sum();
                let mut ray_results: Vec<RayResult> = Vec::new();
                rays.origins.axis_iter(Axis(0)).zip(rays.directions.axis_iter(Axis(0))).enumerate().for_each(|(index, (orig, dir))| {
                    let no = (&n * &orig).sum();
                    let nd = (&n * &dir).sum();
                    let t = (-no - d) / nd;

                    
                    if t >= 0.0 {
                        ray_results.push(RayResult { id: index, hit_point: &orig + t * &dir, dist: t, normal: n.clone() })
                    }
                });

                ray_results
                
            },
            Object::Cube(object_data, _) => panic!("Cube not implemented"),
        }
    } 
}


pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn in_other_frame(self: &Self, wTc: &Transform) -> Self {
        let cTw = wTc.inverse();
        Self {
            objects: self.objects.iter().map(|obj| {
                match obj {
                    Object::Sphere(object_data, f) => Object::Sphere(object_data.transform(&cTw), *f),
                    Object::Plane(object_data) => Object::Plane(object_data.transform(&cTw)),
                    
                    Object::Cube(object_data, f) => Object::Cube(object_data.transform(&cTw), *f),
                }
            }).collect()
        }
    }

    pub fn base_scene() -> Self {
        let mut objects = Vec::<Object>::new();
        let t = Transform::new().with_new_position(Vec3::new(0.0, 0.0, 1.0));

        objects.push(Object::sphere(t, 0.5));

        let t: Transform = 
            &Transform::rotate_y(180.0_f64.to_radians()) * &Transform::new().with_new_position(Vec3::new(0.0, 0.0, -1.5));
        objects.push(Object::Plane(ObjectData {
            pose: t,
            color: Color { r: 0.0, g: 1.0, b: 0.0}
        }));
        Scene {
            objects: objects
        }
    }
}
