use core::f64;

use ndarray::{Array1, s};
use ndarray_linalg::Norm;

use crate::core::{camera::{RayResult, Rays}, color::Color, math::QuadraticSolutions, transform::Transform};

pub struct ObjectData {
    pub pose: Transform,
    pub color: Color,
}

impl ObjectData {
    pub fn transform(self: &Self, t: &Transform) -> Self {
        ObjectData {
            pose: &self.pose * t,
            color: self.color,
        }
    }
}

pub enum Object {
    Sphere(ObjectData, f64),
    Plane(ObjectData),
    Box(ObjectData, (f64, f64, f64)),
}


impl Object {
    pub fn color(self: &Self) -> Color {
        match self {
            Object::Sphere(d, _) | Object::Plane(d) | Object::Box(d, _) => d.color,
        }
    }
    pub fn sphere(pose: Transform, radius: f64) -> Object {
        Object::Sphere(
            ObjectData {
                pose: pose,
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                },
            },
            radius,
        )
    }
    pub fn intersect(self: &Self, rays: &Rays) -> Vec<RayResult> {
        match self {
            Object::Sphere(object_data, radius) => {
                let new_rays = rays.into_other_frame(&object_data.pose);
                println!("{new_rays:?}");
                let solutions = new_rays.iter().map(|(o, d)| {
                    let a = 1.0;

                    let b = ((&d * 2.0) * &o).sum();
                    let c = (o.pow2()).sum() - (radius * radius);
                    let disc = b * b - 4.0 * a * c;
                    match disc {
                        ..0.0 => QuadraticSolutions::None,
                        0.0 => {
                            let s = -b / (2.0 * a);
                            if s >= 0.0 {
                                QuadraticSolutions::One(s)
                            } else {
                                QuadraticSolutions::None
                            }
                        }
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
                        }
                        _ => panic!("This case should not happen"),
                    }
                });
                let center = object_data.pose.position().to_ndarray();
                solutions
                    .enumerate()
                    .filter(|s| match s.1 {
                        QuadraticSolutions::None => false,
                        _ => true,
                    })
                    .map(|(id, solution)| {
                        let t = match solution {
                            QuadraticSolutions::One(s) => s,
                            QuadraticSolutions::Two(s, _) => s,
                            _ => panic!("Cases with no solutions should have been filtered"),
                        };
                        let hit_point = &rays.origins.slice(s![id, ..])
                            + &rays.directions.slice(s![id, ..]) * t;
                        let mut normal = &hit_point - &center;
                        let norm = normal.norm_l2();
                        normal /= norm;
                        println!("Hit point {hit_point:?}");
                        RayResult {
                            id: id,
                            hit_point: hit_point,
                            dist: t,
                            normal: normal,
                        }
                    })
                    .collect()
            }
            Object::Plane(data) => {
                println!("data.pose {0:?}", data.pose);
                let n = data.pose.forward().to_ndarray();
                let d = (data.pose.position().to_ndarray() * &n).sum();
                let mut ray_results: Vec<RayResult> = Vec::new();
                println!("Plane normal: {n:?}");
                rays.into_other_frame(&data.pose).iter().enumerate().for_each(|(index, (orig, dir))| {
                    let no = (&n * &orig).sum();
                    let nd = (&n * &dir).sum();
                    let t = (-no - d) / nd;
                    if t >= 0.0 {
                        ray_results.push(RayResult {
                            id: index,
                            hit_point: &orig + t * &dir,
                            dist: t,
                            normal: n.clone(),
                        })
                    }
                });

                ray_results
            }
            Object::Box(data, dims) => {
                let dim_arr = [dims.0, dims.1, dims.2];
                let mut ray_results: Vec<RayResult> = Vec::new();
                rays.into_other_frame(&data.pose)
                    .iter()
                    .enumerate()
                    .for_each(|(index, (orig, dir))| {
                        let mut tmin = -f64::INFINITY;
                        let mut tmax = f64::INFINITY;
                        let mut n = Array1::<f64>::default(3);
                        for i in 0..3 {
                            let li = dim_arr[i];
                            let oi = orig[i];
                            let di = dir[i];

                            if di == 0.0 {
                                continue;
                            }

                            let t1 = (li - oi) / di;
                            let t2 = (-li - oi) / di;

                            let (t1, t2) = if t2 < t1 {
                                (t2, t1)
                            } else {
                                (t1, t2)
                            };

                            tmin = t1.max(tmin);
                            tmax = t2.min(tmax);
                        }

                        if tmax >= tmin && tmin > 0.0 {
                            ray_results.push(RayResult {
                                id: index,
                                hit_point: &orig + tmin * &dir,
                                dist: tmin,
                                normal: n,
                            });
                        }
                    });

                ray_results
            }
        }
    }
}
