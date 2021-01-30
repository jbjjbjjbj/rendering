mod sphere;

pub use sphere::Sphere;

use crate::core::Ray;
use crate::Float;

trait Shape {
    // 
    fn intersect(ray: Ray) -> Float;
    fn intersect_
}
