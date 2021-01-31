mod sphere;

pub use sphere::Sphere;

use crate::core::Ray;
use crate::Float;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Float>;
}
