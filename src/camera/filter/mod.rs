use crate::vector::Vector2f;
use crate::Float;

pub trait Filter {
    fn eval(&self, point: &Vector2f) -> Float;
    fn radius(&self) -> Vector2f;
}

pub struct BoxFilter {
    radius: Vector2f,
}

// The same a no filter, and can give aliasing in final image
impl Filter for BoxFilter {
    fn eval(&self, _: &Vector2f) -> Float {
        1.0
    }

    fn radius(&self) -> Vector2f {
        self.radius
    }
}
