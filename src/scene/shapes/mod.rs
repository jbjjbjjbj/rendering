mod sphere;

pub use sphere::Sphere;

use crate::core::{Vector3f, Ray};
use crate::Float;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Float>;

    /// Calculates the normal at point
    ///
    /// Point is assumed to be on the circle.
    /// The resulting vector is assumed to be normalized.
    fn norm_at(&self, point: &Vector3f) -> Vector3f;
}
