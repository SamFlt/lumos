
use ndarray::{Array1, Array2, Array3, Axis, Dim};
use ndarray_linalg::{NormalizeAxis};

use crate::core::transform::{Transform, Vec3};
#[derive(Clone)]
pub struct Camera {
    pub pose: Transform,
    pub focal_length: f64,
    pub sensor_width: f64,
    pub sensor_height: f64,
    pub width_resolution: u32,
    pub height_resolution: u32,
}

impl Camera {
    // Grid sampling in the case of a camera without distortion
    fn sample_image_grid(self: &Self) -> ndarray::Array3<f64> {
        let shape: (usize, usize, usize) = (
            self.height_resolution.try_into().unwrap(),
            self.width_resolution.try_into().unwrap(),
            3usize,
        );
        let mut array = Array3::<f64>::zeros(shape);

        let image_plane_corner = Vec3::new(
            -self.sensor_width / 2.0,
            -self.sensor_height / 2.0,
            self.focal_length,
        );

        let pixel_height = self.sensor_height / self.height_resolution as f64;
        let pixel_width = self.sensor_width / self.width_resolution as f64;

        for v in 0..self.height_resolution as usize {
            let y = pixel_height / 2.0 + pixel_height * v as f64;
            for u in 0..self.width_resolution as usize {
                let x = pixel_width / 2.0 + pixel_width * u as f64;
                array[[v, u, 0]] = image_plane_corner.x + x;
                array[[v, u, 1]] = image_plane_corner.y + y;
                array[[v, u, 2]] = image_plane_corner.z;
            }
        }
        array
    }

    pub fn get_rays(self: &Camera) -> Rays {
        let image_grid_points = self.sample_image_grid();
        let num_points = image_grid_points.shape()[0] * image_grid_points.shape()[1];
        let mut pixel_positions = Array3::<usize>::zeros((
            self.height_resolution as usize,
            self.width_resolution as usize,
            2,
        ));
        pixel_positions
            .indexed_iter_mut()
            .for_each(|((u, v, i), val)| {
                if i == 0 {
                    *val = u;
                } else {
                    *val = v;
                }
            });
        let pixel_positions = pixel_positions
            .into_shape_with_order((num_points, 2))
            .expect("Could not reshape pixel positions");

        let positions = Array2::<f64>::zeros((num_points, 3));
        let image_grid_points = image_grid_points
            .into_shape_with_order((num_points, 3))
            .expect("Could not cast into flattened ray array");
        let (directions, _) = ndarray_linalg::normalize(image_grid_points, NormalizeAxis::Row);
        // let norms: Vec<f64> = directions.axis_iter(Axis(0)).map(|s| s.norm_l2()).collect();
        // println!("{norms:?}");
        // panic!();
        Rays {
            origins: positions,
            directions: directions,
            pixel_positions: pixel_positions,
            world_t_ray: self.pose.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rays {
    pub origins: Array2<f64>,
    pub directions: Array2<f64>,
    pub pixel_positions: Array2<usize>,
    pub world_t_ray: Transform,
}

impl Rays {
    pub fn iter(
        self: &Self,
    ) -> std::iter::Zip<
        ndarray::iter::AxisIter<'_, f64, Dim<[usize; 1]>>,
        ndarray::iter::AxisIter<'_, f64, Dim<[usize; 1]>>,
    > {
        let orig_iter = self.origins.axis_iter(Axis(0));
        let dir_iter = self.directions.axis_iter(Axis(0));
        orig_iter.zip(dir_iter)
    }

    pub fn into_other_frame(self: &Self, world_t_other: &Transform) -> Self {
        let projection: Transform = &world_t_other.inverse() * &self.world_t_ray;
        Rays {
            origins: projection.transform(&self.origins),
            directions: projection.rotate(&self.directions),
            pixel_positions: self.pixel_positions.clone(),
            world_t_ray: world_t_other.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RayResult {
    pub id: usize,
    pub hit_point: Array1<f64>,
    pub dist: f64,
    pub normal: Array1<f64>,
}
