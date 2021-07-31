//! The ray class used when probing the 3d scene
use crate::core::Vector3f;
use crate::Float;

/// A ray that is sent into the world.
/// This is the main type used for testing intersections.
pub struct Ray {
    /// Origin of the ray
    pub origin: Vector3f,
    /// Direction is assumed to be a unit vector.
    pub direction: Vector3f,
}

impl Ray {
    pub fn new(origin: Vector3f, direction: Vector3f) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn new_to(origin: Vector3f, target: Vector3f) -> Ray {
        let dir = (target - origin).norm();
        Ray {
            origin,
            direction: dir
        }
    }

    /// Resolve a point on the ray at time t
    pub fn at(&self, t: Float) -> Vector3f {
        self.origin + self.direction * t
    }
}

