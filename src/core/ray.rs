//! The ray class used when probing the 3d scene
use crate::core::Vector3f;
use crate::Float;

pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn at(&self, t: Float) -> Vector3f {
        self.origin + self.direction * t
    }
}

