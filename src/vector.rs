use crate::{Float, Number};
use std::ops::{Sub};

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

    pub fn from_xy(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T: Number> Sub for Vector2<T> {
    type Output = Self;
    fn sub(self, op: Self) -> Self::Output {
        Vector2 {
            x: self.x - op.x,
            y: self.y - op.y,
        }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec2() {
        let v = Vector2::from_xy(2.0, 10.0);

        assert!(v.x == 2.0 && v.y == 10.0);

        let v = Vector2::new(3);

        assert!(v.x == 3 && v.y == 3);
    }

    #[test]
    fn sub_vec2() {
        let v1 = Vector2::from_xy(10, 11);
        let v2 = Vector2::from_xy(2, 3);

        let v3 = v1-v2;
        assert!(v3.x == 8 && v3.y == 8);
    }
}
