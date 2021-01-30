use crate::Float;
use crate::core::{Ray, Vector3f};

pub struct Sphere {
    radius: Float,
    center: Vector3f,
}

impl Sphere {
    pub fn new(radius: Float, center: Vector3f) -> Sphere {
        Sphere {
            radius,
            center,
        }
    }
}
