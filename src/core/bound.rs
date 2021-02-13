//! Implements a 2d region
use crate::{Number, Float};
use super::vector2::Vector2;
use crate::core;

/// Implements a region between min and max
#[derive(Clone)]
pub struct Bound2<T: Number> {
    pub min: Vector2<T>,
    pub max: Vector2<T>
}

pub type Bound2i = Bound2<i32>;
pub type Bound2f = Bound2<Float>;

fn min<T: Number> (a: T, b: T) -> T {
    if b < a {
        return b;
    }
    a
}

fn max<T: Number> (a: T, b: T) -> T {
    if b > a {
        return b;
    }
    a
}

impl<T: Number> Bound2<T> {
    /// Creates a new bound from two points
    ///
    /// p0 does not have to be smaller than p1
    pub fn new(p0: &Vector2<T>, p1: &Vector2<T>) -> Self {
        let min = Vector2::new_xy(min(p0.x, p1.x), min(p0.y, p1.y));
        let max = Vector2::new_xy(max(p0.x, p1.x), max(p0.y, p1.y));

        Self { min, max }
    }

    pub fn new_xyxy(x1: T, y1: T, x2: T, y2: T) -> Self {
        Self::new(
            &Vector2::new_xy(x1, y1),
            &Vector2::new_xy(x2, y2),
            )
    }

    /// Finds the intersected area between two bounds
    pub fn intersect(&self, b: &Bound2<T>) -> Bound2<T> {
        Bound2::new(
            &Vector2::new_xy(max(self.min.x, b.min.x), max(self.min.y, b.min.y)),
            &Vector2::new_xy(min(self.max.x, b.max.x), min(self.max.y, b.max.y)),
            )
    }


    /// Calculates the diagonal vector
    ///
    /// Can be used to calculate the size of the bound
    ///
    /// # Examples
    ///
    /// ```
    /// use rendering::core::Bound2i;
    /// let b = Bound2i::new_xyxy(2, 2, 6, 7);
    /// let diag = b.diagonal();
    ///
    /// assert!(diag.x == 4 && diag.y == 5);
    /// ```
    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    /// Calculates the area of of the bounded region
    ///
    /// # Examples
    ///
    /// ```
    /// use rendering::core::Bound2i;
    /// let b = Bound2i::new_xyxy(10, 10, 20, 20);
    /// 
    /// assert!(b.area() == 100);
    /// ```
    pub fn area(&self) -> T {
        let diag = self.diagonal();
        diag.x * diag.y
    }
}

impl From<&Bound2i> for Bound2f {
    fn from(b: &Bound2i) -> Self {
        Self {
            min: core::Vector2f::from(b.min),
            max: core::Vector2f::from(b.max),
        }
    }
}

impl From<&Bound2f> for Bound2i {
    fn from(b: &Bound2f) -> Self {
        Self {
            min: core::Vector2i::from(b.min),
            max: core::Vector2i::from(b.max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test() -> Bound2<i32> {
        Bound2::new(
            &Vector2::new_xy(1, 2),
            &Vector2::new_xy(10, 3)
            )
    }

    #[test]
    fn area() {
        let b = create_test();

        assert!(b.area() == 9);
    }

    #[test]
    fn intersect_test() {
        let b1 = Bound2i::new_xyxy(10, 10, 20, 20);
        let b2 = Bound2i::new_xyxy(2, 11, 22, 17);

        let b = b1.intersect(&b2);

        assert!(
            b.min.x == 10 &&
            b.min.y == 11 &&
            b.max.x == 20 &&
            b.max.y == 17
            )
    }
}
