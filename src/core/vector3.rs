//! Implements 3d vectors
//!
//! Also add more 3d math things needed for shading and 3d calculations.
use crate::{Float, Number};
use std::ops::{Mul, Sub, Add, DivAssign, Neg};
use std::fmt;

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

    pub fn new_xyz(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z}
    }
}

impl<T: Number> Sub for Vector3<T> {
    type Output = Self;
    fn sub(self, op: Self) -> Self::Output {
        Self::new_xyz(
            self.x - op.x,
            self.y - op.y,
            self.z - op.z,
        )
    }
}

impl<T: Number> Add for Vector3<T> {
    type Output = Self;
    fn add(self, op: Self) -> Self::Output {
        Self::new_xyz(
            self.x + op.x,
            self.y + op.y,
            self.z + op.z,
        )
    }
}

impl<T: Number> Add<T> for Vector3<T> {
    type Output = Self;

    fn add(self, op: T) -> Self::Output {
        Self::new_xyz(
            self.x + op,
            self.y + op,
            self.z + op,
        )
    }
}

impl<T: Number> Mul<T> for Vector3<T> {
    type Output = Self;
    fn mul(self, op: T) -> Self::Output {
        Self::Output::new_xyz(
            self.x * op,
            self.y * op,
            self.z * op,
            )
    }
}

impl<T: Number> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new_xyz(
            -self.x,
            -self.y,
            -self.z,
            )
    }
}

impl<T: Number> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, op: T) {
        self.x /= op;
        self.y /= op;
        self.z /= op;
    }
}

impl<T: Number> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[{}, {}, {}]", self.x, self.y, self.z))
    }
}

impl Vector3f {
    /// Calculates the length times itself
    ///
    /// This is faster than using len * len as the square is ommited
    pub fn len_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> Float {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, op: &Self) -> Float {
        self.x * op.x + self.y * op.y + self.z * op.z
    }

    /// Inplace normal instead of creating a new vector
    ///
    /// # Example
    ///
    /// ```
    /// use pathtrace::core::Vector3f;
    /// let mut v = Vector3f::new_xyz(10.0, 0.0, 0.0);
    /// v.norm_in();
    /// assert!(v.x == 1.0);
    /// ```
    pub fn norm_in(&mut self) {
        // TODO Experiment with checking for normality with len_squared
        let len = self.len();
        if len == 0.0 {
            *self = Self::new(0.0);
        }

        *self /= len;
    }

    pub fn norm(&self) -> Self {
        let mut new = self.clone();
        new.norm_in();
        new
    }

    pub fn cross(&self, op: &Self) -> Self {
        Self::new_xyz(
            self.y * op.z - self.z * op.y,
            self.z * op.x - self.x * op.z,
            self.x * op.y - self.y * op.x,
            )

    }
}
