use ndarray::{Array2, Array3};
use ndarray_linalg::NormalizeAxis;

use crate::transform::{Transform, Vec3};
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
                array[[v, u, 0]] = x;
                array[[v, u, 1]] = y;
                array[[v, u, 2]] = image_plane_corner.z;
            }
        }
        array
    }

    fn get_rays(self) -> Rays {
        let image_grid_points = self.sample_image_grid();
        let num_points = image_grid_points.shape()[0] * image_grid_points.shape()[1];
        let positions = Array2::<f64>::zeros((num_points, 3));
        let image_grid_points = image_grid_points.to_shape((num_points, 3)).expect("Could not cast into flattened ray array").to_owned();

        Rays {
            positions: positions,
            directions: ndarray_linalg::normalize(image_grid_points, NormalizeAxis::Column).0
        }
    }
}

pub struct Rays {
    pub positions: Array2<f64>,
    pub directions: Array2<f64>

}
