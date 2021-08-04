//! Implements 3d vectors
//!
//! Also add more 3d math things needed for shading and 3d calculations.
use crate::{Float, Number, NEAR_ZERO};
use std::ops::{Mul, Sub, Add, Div, DivAssign, Neg, AddAssign, Index};
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

impl<T: Number> Div<T> for Vector3<T> {
    type Output = Self;
    fn div(self, op: T) -> Self::Output {
        Self::Output::new_xyz(
            self.x / op,
            self.y / op,
            self.z / op,
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

impl<T: Number> AddAssign<&Self> for Vector3<T> {
    fn add_assign(&mut self, op: &Self) {
        self.x += op.x;
        self.y += op.y;
        self.z += op.z;
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

// Ohh god
impl<T: Number> Index<u32> for Vector3<T> {
    type Output = T;

    fn index(&self, i: u32) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: index {} is not possible with 3d vector", i)
        }
    }
}

impl Vector3f {
    pub const ZERO: Self = Vector3f {x: 0.0, y: 0.0, z: 0.0};

    /// Calculates the length times itself
    ///
    /// This is faster than using len * len as the square is ommited
    pub fn len_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> Float {
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
    /// use rendering::core::Vector3f;
    /// let mut v = Vector3f::new_xyz(10.0, 0.0, 0.0);
    /// v.norm_in();
    /// assert!(v.x == 1.0);
    /// ```
    pub fn norm_in(&mut self) {
        // TODO Experiment with checking for normality with len_squared
        let len = self.length();
        if len == 0.0 {
            *self = Self::new(0.0);
        }

        *self /= len;
    }

    pub fn norm(&self) -> Self {
        let mut new = *self;
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

    /// Check if vector is close to [0, 0, 0]
    ///
    /// This is based on the NEAR_ZERO constant
    pub fn near_zero(&self) -> bool {
        (self.x.abs() < NEAR_ZERO) &&
            (self.y.abs() < NEAR_ZERO) &&
            (self.z.abs() < NEAR_ZERO)
    }
}
