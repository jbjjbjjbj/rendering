//! Implements 3d axis aligned bounding box
use crate::{Number, Float};
use super::vector3::{Vector3, Vector3f};
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

    pub fn combine(&self, op: &Self) -> Self {
        let min = Vector3::new_xyz(
            min(self.min.x, op.min.x),
            min(self.min.y, op.min.y),
            min(self.min.z, op.min.z)
            );
        let max = Vector3::new_xyz(
            max(self.max.x, op.max.x),
            max(self.max.y, op.max.y),
            max(self.max.z, op.max.z)
            );

        Self {min, max}
    }

    pub fn and(&self, op: &Self) -> Self {
        let min_b = Vector3::new_xyz(
            max(self.min.x, op.min.x),
            max(self.min.y, op.min.y),
            max(self.min.z, op.min.z)
            );
        let max_b = Vector3::new_xyz(
            min(self.max.x, op.max.x),
            min(self.max.y, op.max.y),
            min(self.max.z, op.max.z)
            );

        Self {min: min_b, max: max_b}
    }

    pub fn area(&self) -> T {
        let diag = self.max - self.min;
        diag.x * diag.y * diag.z
    }
}

impl Bound3f {
    pub const EMPTY: Bound3f = Bound3f{min: Vector3f::ZERO, max: Vector3f::ZERO};

    /// Calculate whether there is a intersect between a bounding box and a ray
    ///
    /// # Examples:
    ///
    /// ```
    /// use rendering::core::{Bound3f, Vector3f, Ray};
    /// use rendering::INFTY;
    /// let b = Bound3f::new(Vector3f::new(7.0), Vector3f::new(10.0));
    /// let r1 = Ray::new_to(Vector3f::new(0.0), Vector3f::new(5.0));
    /// let r2 = Ray::new_to(Vector3f::new(-0.0), Vector3f::new(-5.0));
    /// let r3 = Ray::new(Vector3f::new_xyz(-1.0, 0.0, 0.0), Vector3f::new_xyz(1.0, 0.0, 0.0));
    ///
    /// assert!(b.intersect(&r1, 0.0, INFTY));
    /// assert!(!b.intersect(&r2, 0.0, INFTY));
    /// assert!(!b.intersect(&r3, 0.0, INFTY));
    /// ```
    pub fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        println!("BIN: {} -> {}", self.min, self.max);
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

            let t_min = max(t0, t_min);
            let t_max = min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }
}
