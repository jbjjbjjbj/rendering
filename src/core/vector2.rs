//! Implements 2d vectors
//!
//! This is implemented generictly with types that fit in the Number trait
use crate::{Float, Number};
use std::ops::{Sub, Add, Mul};
use std::fmt;
use std::cmp::min;

#[derive(Clone, Copy)]
pub struct Vector2<T: Number> {
    pub x: T,
    pub y: T,
}

pub type Vector2f = Vector2<Float>;
pub type Vector2i = Vector2<i32>;

impl<T: Number> Vector2<T> {
    pub fn new(initial: T) -> Vector2<T> {
        Vector2 { x: initial, y: initial }
    }

    pub fn new_xy(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T: Number> Sub for Vector2<T> {
    type Output = Self;
    fn sub(self, op: Self) -> Self::Output {
        Self::new_xy(
            self.x - op.x,
            self.y - op.y,
        )
    }
}

impl<T: Number> Add for Vector2<T> {
    type Output = Self;
    fn add(self, op: Self) -> Self::Output {
        Self::new_xy(
            self.x + op.x,
            self.y + op.y,
        )
    }
}

impl<T: Number> Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(self, op: T) -> Self::Output {
        Self::new_xy(
            self.x * op,
            self.y * op,
        )
    }
}

impl<T: Number> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}

impl Vector2f {
    pub fn ceil(&self) -> Self {
        Self::new_xy(
            self.x.ceil(),
            self.y.ceil()
            )
    }

    pub fn floor(&self) -> Self {
        Self::new_xy(
            self.x.floor(),
            self.y.floor()
            )
    }
}

impl From<Vector2i> for Vector2f {
    fn from(v: Vector2i) -> Self {
        Self {
            x: v.x as Float,
            y: v.y as Float,
        }
    }
}

impl From<Vector2f> for Vector2i {
    fn from(v: Vector2f) -> Self {
        Self {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

impl Vector2i {
    pub fn cap(&self, x: i32, y: i32) -> Self {
        Self::new_xy(
            min(self.x, x),
            min(self.y, y),
            )
    }
}

impl Vector2f {
    pub fn len(&self) -> Float {
        (self.x*self.x + self.y*self.y).sqrt()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec2() {
        let v = Vector2::new_xy(2.0, 10.0);

        assert!(v.x == 2.0 && v.y == 10.0);

        let v = Vector2::new(3);

        assert!(v.x == 3 && v.y == 3);
    }

    #[test]
    fn sub_vec2() {
        let v1 = Vector2::new_xy(10, 11);
        let v2 = Vector2::new_xy(2, 3);

        let v3 = v1-v2;
        assert!(v3.x == 8 && v3.y == 8);
    }

    #[test]
    fn add_vec2() {
        let v1 = Vector2::new_xy(10, 11);
        let v2 = Vector2::new_xy(2, 3);

        let v3 = v1+v2;
        assert!(v3.x == 12 && v3.y == 14);
    }
}
