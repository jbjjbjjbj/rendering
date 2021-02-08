use crate::core::{Vector3f, Ray};
use crate::Float;

/// Returns the context of a intersection
pub struct Intersection {
    /// Normal vector at intersection
    pub n: Vector3f,
    pub p: Vector3f,
    pub t: Float,
}

impl Intersection {
    pub fn norm_against_ray(&self, r: &Ray) -> Vector3f {
        if self.n.dot(&r.direction) < 0.0 {
            self.n
        } else {
            -self.n
        }
    }
}

/// Defines a common trait for objects in the scene
pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
