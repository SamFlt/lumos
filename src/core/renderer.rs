use ndarray::{Array3, s};
use ndarray_linalg::Norm;

use crate::core::camera::{Camera, RayResult};
use crate::core::color::Color;
use crate::core::light::Light;
use crate::core::object::Object;
use crate::core::renderer_settings::{RenderType, RendererSettings};
use crate::core::scene::Scene;
use crate::core::transform::Transform;

pub struct LumosRenderer {
    pub camera: Camera,
    pub scene: Scene,
    pub settings: RendererSettings,
}

impl Default for LumosRenderer {
    fn default() -> Self {
        Self {
            camera: Camera {
                pose: Transform::new(),
                focal_length: 0.05,
                sensor_width: 0.05,
                sensor_height: 0.05,
                width_resolution: 800,
                height_resolution: 800,
            },
            scene: Scene::base_scene(),
            settings: RendererSettings::default(),
        }
    }
}

impl LumosRenderer {
    pub fn render(self: &Self) -> Array3<u8> {
        let (h, w) = (
            self.camera.height_resolution as usize,
            self.camera.width_resolution as usize,
        );
        let mut image = Array3::<u8>::default((h, w, 4));
        image.slice_mut(s![.., .., 3]).fill(255); // Alpha channel
        // let scene = self.scene.in_other_frame(&self.camera.pose);

        let rays = self.camera.get_rays();
        let mut closest_rays: Vec<Option<(usize, RayResult)>> = Vec::new();
        closest_rays.resize_with(h * w, || None);
        self.scene
            .objects
            .iter()
            .enumerate()
            .for_each(|(obj_id, obj)| {
                let ray_hits = obj.intersect(&rays);
                for ray in ray_hits {
                    let pixel_position = rays.pixel_positions.row(ray.id);
                    let current_value =
                        &mut closest_rays[pixel_position[0] * w + pixel_position[1]];
                    let new_value = Some((obj_id, ray));
                    *current_value = match current_value {
                        None => new_value,
                        Some((u, r)) => {
                            if r.dist < new_value.as_ref().unwrap().1.dist {
                                Some((*u, r.clone()))
                            } else {
                                new_value
                            }
                        }
                    };
                }
            });
        match self.settings.render_type {
            RenderType::Depth => {
                let max_depth = closest_rays.iter().fold(0.0, |acc, v| match v {
                    None => acc,
                    Some((_, ray)) => {
                        if ray.dist > acc {
                            ray.dist
                        } else {
                            acc
                        }
                    }
                });
                closest_rays
                    .iter()
                    .enumerate()
                    .for_each(|(pixel_index, pixel_result)| {
                        let v = pixel_index / w;
                        let u = pixel_index % w;
                        match pixel_result {
                            None => {
                                image[[v, u, 0]] = 255;
                                image[[v, u, 1]] = 255;
                                image[[v, u, 2]] = 255;
                            }
                            Some((_, ray_result)) => {
                                let val = (ray_result.dist / max_depth * 255.0)
                                    .clamp(0.0, 255.0)
                                    .round() as u8;

                                image[[v, u, 0]] = val;
                                image[[v, u, 1]] = val;
                                image[[v, u, 2]] = val;
                            }
                        }
                    });
            }
            RenderType::Normals => todo!(),
            RenderType::Color => {
                closest_rays
                    .iter()
                    .enumerate()
                    .for_each(|(pixel_index, pixel_result)| {
                        let v = pixel_index / w;
                        let u = pixel_index % w;
                        match pixel_result {
                            None => {
                                image[[v, u, 0]] = 0;
                                image[[v, u, 1]] = 0;
                                image[[v, u, 2]] = 0;
                            }
                            Some((obj_id, ray_result)) => {
                                let obj: &Object = &(self.scene.objects[*obj_id]);

                                let mut c = Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                };

                                for light in &self.scene.lights {
                                    match light {
                                        Light::Point(pos) => {
                                            let d = pos.to_ndarray() - &ray_result.hit_point;
                                            let dnorm = d.norm_l2();
                                            let d = d / dnorm;
                                            let coeff = (d * &ray_result.normal).sum();
                                            c += obj.color() * coeff as f32;
                                        }
                                        Light::Directional(dir) => todo!(),
                                    }
                                }

                                image[[v, u, 0]] = (c.r * 255.0).round() as u8;
                                image[[v, u, 1]] = (c.g * 255.0).round() as u8;
                                image[[v, u, 2]] = (c.b * 255.0).round() as u8;
                            }
                        }
                    });
            }
        }

        image
    }
}

// println!("{closest_rays:?}");

// let r = rays.pixel_positions.map_axis(ndarray::Axis(1), |px| px[0] as f32 / h as f32);
// let g = rays.pixel_positions.map_axis(ndarray::Axis(1), |px| px[1] as f32 / w as f32);
// let b = Array2::<u8>::zeros((h, w));
// println!("{r}");
// let [r, g] = [r,g].map(|arr_float| arr_float.into_shape_with_order((h, w)).unwrap().map(|vf| (vf * 255.0) as u8));
// for (i, channel) in [r,g].iter().enumerate() {
//     image.slice_mut(s![.., .., i]).assign(&channel);
// }
