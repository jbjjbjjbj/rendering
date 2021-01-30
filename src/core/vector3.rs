use crate::{Float, Number};
use std::ops::{Sub, Add};

#[derive(Clone, Copy)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vector3f = Vector3<Float>;

impl<T: Number> Vector3<T> {
    pub fn new(initial: T) -> Vector3<T> {
        Vector3 { 
            x: initial,
            y: initial,
            z: initial,
        }
    }

    pub fn new_xy(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z}
    }
}

impl<T: Number> Sub for Vector3<T> {
    type Output = Self;
    fn sub(self, op: Self) -> Self::Output {
        Self::new_xy(
            self.x - op.x,
            self.y - op.y,
            self.z - op.z,
        )
    }
}

impl<T: Number> Add for Vector3<T> {
    type Output = Self;
    fn add(self, op: Self) -> Self::Output {
        Self::new_xy(
            self.x + op.x,
            self.y + op.y,
            self.z + op.z,
        )
    }
}
