use std::thread::current;

use ndarray::{Array2, Array3, s};

use crate::core::camera::{Camera, RayResult};
use crate::core::scene::{Object, Scene};
use crate::core::transform::Transform;

pub struct LumosRenderer {
    pub camera: Camera,
    pub scene: Scene
}

impl Default for LumosRenderer {
    fn default() -> Self {
        Self { camera: Camera {
                pose: Transform::new(),
                focal_length: 0.05,
                sensor_width: 0.05,
                sensor_height: 0.05,
                width_resolution: 1200,
                height_resolution: 800,
            }, scene: Scene::base_scene() }
    }
}

impl LumosRenderer {

    pub fn render(self: &Self) -> Array3<u8> {
        let (h, w) = (self.camera.height_resolution as usize, self.camera.width_resolution as usize);
        let mut image = Array3::<u8>::default((h, w, 4));
        image.slice_mut(s![.., .., 3]).fill(255); // Alpha channel
        let scene = self.scene.in_other_frame(&self.camera.pose);

        let rays = self.camera.get_rays();
        let mut closest_rays: Vec<Option<(usize, RayResult)>> = Vec::new();
        closest_rays.resize_with(h * w, || {None});
        scene.objects.iter().enumerate().for_each(|(obj_id, obj)| {
            let ray_hits = obj.intersect(&rays);
            for ray in ray_hits {
                let pixel_position = rays.pixel_positions.row(ray.id);
                let current_value = &mut closest_rays[pixel_position[0] * w + pixel_position[1]];
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


        closest_rays.iter().enumerate().for_each(|(pixel_index, pixel_result)| {
            let v = pixel_index / w;
            let u = pixel_index % w;
            match pixel_result {
                None => {
                    image[[v, u, 0]] = 0;
                    image[[v, u, 1]] = 0;
                    image[[v, u, 2]] = 0;
                },
                Some((obj_id, ray_result)) => {
                    let obj: &Object = &(scene.objects[*obj_id]);
                    let c = obj.color();
                    image[[v, u, 0]] = (c.r * 255.0).round() as u8;
                    image[[v, u, 1]] = (c.g * 255.0).round() as u8;
                    image[[v, u, 2]] = (c.b * 255.0).round() as u8;
                    
                }
            }
        });

        // println!("{closest_rays:?}");


        // let r = rays.pixel_positions.map_axis(ndarray::Axis(1), |px| px[0] as f32 / h as f32);
        // let g = rays.pixel_positions.map_axis(ndarray::Axis(1), |px| px[1] as f32 / w as f32);
        // let b = Array2::<u8>::zeros((h, w));
        // println!("{r}");
        // let [r, g] = [r,g].map(|arr_float| arr_float.into_shape_with_order((h, w)).unwrap().map(|vf| (vf * 255.0) as u8));
        // for (i, channel) in [r,g].iter().enumerate() {
        //     image.slice_mut(s![.., .., i]).assign(&channel);
        // }
        

        image
    }

    
}
