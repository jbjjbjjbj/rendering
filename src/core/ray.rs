//! The ray class used when probing the 3d scene
use crate::core::Vector3f;
use crate::Float;

pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn new_to(origin: Vector3f, target: Vector3f) -> Ray {
        let dir = (target - origin).norm();
        Ray {
            origin,
            direction: dir
        }
    }

    pub fn at(&self, t: Float) -> Vector3f {
        self.origin + self.direction * t
    }
}

