use crate::core::{Vector3f, Bound3f, Ray};
use crate::Float;
use crate::material::Material;

/// Returns the context of a intersection
pub struct Intersection<'a> {
    /// Normal vector at intersection
    pub n: Vector3f,
    pub p: Vector3f,
    pub t: Float,
    pub m: Option<&'a dyn Material>,
}

impl Intersection<'_> {
    pub fn norm_against_ray(&self, r: &Ray) -> Vector3f {
        if self.n.dot(&r.direction) < 0.0 {
            self.n
        } else {
            -self.n
        }
    }
}

/// Defines a common trait for objects in the scene
pub trait Hittable: Sync + Send {
    /// Returns the intersection with ray
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;

    /// Returns the axis alligned bounding box containing self
    fn bounding_box(&self) -> Option<Bound3f> {
        None
    }
}
