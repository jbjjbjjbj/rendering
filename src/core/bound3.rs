//! Implements 3d axis aligned bounding box
use crate::{Number, Float};
use super::vector3::Vector3;
use crate::core::{min, max};
use crate::core::Ray;

#[derive(Clone)]
pub struct Bound3<T: Number> {
    pub min: Vector3<T>,
    pub max: Vector3<T>
}

pub type Bound3f = Bound3<Float>;

impl<T: Number> Bound3<T> {
    /// Creates a bound from two points
    pub fn new(p0: Vector3<T>, p1: Vector3<T>) -> Self {
        // Elliminate some code duplication here
        let min = Vector3::new_xyz(
            min(p0.x, p1.x),
            min(p0.y, p1.y),
            min(p0.z, p1.z)
            );
        let max = Vector3::new_xyz(
            max(p0.x, p1.x),
            max(p0.y, p1.y),
            max(p0.z, p1.z)
            );

        Self {min, max}
    }
}

impl Bound3f {
    /// Calculate whether there is a intersect between a bounding box and a ray
    ///
    /// # Examples:
    ///
    /// ```
    /// use rendering::core::{Bound3f, Vector3f, Ray};
    /// let b = Bound3f::new(Vector3f::new(7.0), Vector3f::new(10.0));
    /// let r1 = Ray::new_to(Vector3f::new(0.0), Vector3f::new(5.0));
    /// let r3 = Ray::new(Vector3f::new_xyz(-1.0, 0.0, 0.0), Vector3f::new_xyz(1.0, 0.0, 0.0));
    ///
    /// assert!(b.intersect(&r1));
    /// assert!(!b.intersect(&r3));
    /// ```
    pub fn intersect(&self, ray: &Ray) -> bool {
        // Method stolen from Ray tracing the next week.
        // They mention its from pixar
        for i in 0..3 {
            let inv = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv;

            if inv < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            if t1 <= t0 {
                return false;
            }
        }

        return true;
    }
}
