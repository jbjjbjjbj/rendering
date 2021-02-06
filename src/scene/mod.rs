//! Defines the scene type which contains all the objects in the scene.
//! 
//! Also handles finding intersections between rays and shapes
pub mod shapes;

mod scene;
pub use scene::*;

use crate::core::{Ray, Vector3f};
use crate::Float;

/// Returns the context of a intersection
pub struct Intersection {
    /// Normal vector at intersection
    pub n: Vector3f,
    pub p: Vector3f,
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
