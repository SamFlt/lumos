use std::ops::{Add, Mul};

use ndarray::{Array1, Array2, ArrayView2, Axis, s};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn to_ndarray(self: &Self) -> ndarray::Array1<f64> {
        Array1::from_vec(vec![self.x, self.y, self.z])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Clone, Debug)]
pub struct Transform {
    pose: ndarray::Array2<f64>,
}

impl Mul<&Transform> for &Transform {
    type Output = Transform;

    fn mul(self, rhs: &Transform) -> Self::Output {
        Transform {
            pose: self.pose.dot(&rhs.pose),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pose: ndarray::Array2::<f64>::eye(4),
        }
    }

    pub fn at_position(position: Vec3) -> Self {
        Self::new().with_new_position(position)
    }

    pub fn rotate_y(rads: f64) -> Transform {
        let mut arr = Array2::<f64>::eye(4);
        let c = rads.cos();
        let s = rads.sin();
        let data =
            Array2::<f64>::from_shape_vec((3, 3), [c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c].to_vec())
                .unwrap();

        arr.slice_mut(s![0..3, 0..3]).assign(&data);
        Transform { pose: arr }
    }

    pub fn position(self: &Self) -> Vec3 {
        Vec3 {
            x: self.pose[[0, 3]],
            y: self.pose[[1, 3]],
            z: self.pose[[2, 3]],
        }
    }

    pub fn with_new_position(self: Self, v: Vec3) -> Self {
        let mut a = self.pose.clone();

        a[[0, 3]] = v.x;
        a[[1, 3]] = v.y;
        a[[2, 3]] = v.z;

        Transform { pose: a }
    }

    pub fn forward(self: &Self) -> Vec3 {
        Vec3 {
            x: self.pose[[0, 2]],
            y: self.pose[[1, 2]],
            z: self.pose[[2, 2]],
        }
    }

    pub fn inverse(self: &Self) -> Transform {
        let r = self.pose.slice(s![0..3, 0..3]);
        let rt = r.t();
        let t: Array1<f64> = self.pose.slice(s![0..3, 3]).to_owned();
        let tinv: Array1<f64> = -(rt.dot(&t));

        let mut np = Array2::<f64>::zeros((4, 4));

        np.slice_mut(s![0..3, 0..3]).assign(&r);
        np.slice_mut(s![0..3, 3]).assign(&tinv);
        np[[3, 3]] = 1.0;

        Transform { pose: np }
    }

    pub fn rotation_matrix(self: &Self) -> ArrayView2<'_, f64> {
        self.pose.slice(s![0..3, 0..3])
    }

    pub fn transform(self: &Self, points: &Array2<f64>) -> Array2<f64> {
        let rot_mat = self.rotation_matrix();
        let t = self.pose.slice(s![0..3, 3]);
        let n = points.shape()[0];
        let mut res: Array2<f64> = Array2::default((n, 3));
        points
            .axis_iter(Axis(0))
            .enumerate()
            .for_each(|(idx, point)| {
                let p = rot_mat.dot(&point) + &t;
                res.row_mut(idx).assign(&p);
            });
        res
    }

    pub fn rotate(self: &Self, vecs: &Array2<f64>) -> Array2<f64> {
        let rot_mat = self.rotation_matrix();
        let n = vecs.shape()[0];
        let mut res: Array2<f64> = Array2::default((n, 3));
        vecs.axis_iter(Axis(0))
            .enumerate()
            .for_each(|(idx, point)| {
                res.row_mut(idx).assign(&rot_mat.dot(&point));
            });
        res
    }
}
