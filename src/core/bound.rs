use crate::{Number, Float};
use super::vector2::Vector2;
use crate::core;

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

    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    pub fn area(&self) -> T {
        let diag = self.diagonal();
        return diag.x * diag.y;
    }

    pub fn width(&self) -> T {
        self.diagonal().x
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

pub fn intersect<T: Number>(a: &Bound2<T>, b: &Bound2<T>) -> Bound2<T> {
    Bound2::new(
        &Vector2::new_xy(max(a.min.x, b.min.x), max(a.min.y, b.min.y)),
        &Vector2::new_xy(min(a.max.x, b.max.x), min(a.max.y, b.max.y)),
        )
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

        let b = intersect(&b1, &b2);

        assert!(
            b.min.x == 10 &&
            b.min.y == 11 &&
            b.max.x == 20 &&
            b.max.y == 17
            )
    }
}
