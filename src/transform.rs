use std::ops::{Add, Mul};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
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

pub struct Transform {
    pose: ndarray::Array2<f64>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pose: ndarray::Array2::<f64>::eye(4),
        }
    }

    pub fn position(self: &Self) -> Vec3 {
        Vec3 {
            x: self.pose[[0, 3]],
            y: self.pose[[1, 3]],
            z: self.pose[[2, 3]],
        }
    }

    pub fn withNewPosition(self: &Self, v: Vec3) -> Self {
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
}
