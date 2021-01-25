use crate::{Number, Float};
use crate::vector::Vector2;
use std::cmp;

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
    fn new(p0: &Vector2<T>, p1: &Vector2<T>) -> Bound2<T> {
        let min = Vector2::from_xy(min(p0.x, p1.x), min(p0.y, p1.y));
        let max = Vector2::from_xy(max(p0.x, p1.x), max(p0.y, p1.y));

        Bound2 { min, max }
    }

    fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    fn area(&self) -> T {
        let diag = self.diagonal();
        return diag.x * diag.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test() -> Bound2<i32> {
        Bound2::new(
            &Vector2::from_xy(1, 2),
            &Vector2::from_xy(10, 3)
            )
    }

    #[test]
    fn area() {
        let b = create_test();

        assert!(b.area() == 9);
    }
}
