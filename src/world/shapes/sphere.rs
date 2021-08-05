//! Implements sphere
//!
//! Spheres are relatively easy to calculate intersections between
use crate::{Float, NEAR_ZERO};
use crate::core::{Ray, Vector3f, Bound3f};
use crate::world::{Hittable, DynHittable, Intersection, Instancable};

pub struct Sphere {
    radius: Float,
}

impl Sphere {
    pub fn new(radius: Float) -> Sphere {
        Sphere {
            radius,
        }
    }

    fn norm_at(&self, point: &Vector3f) -> Vector3f {
        *point / self.radius
    }
}

impl Hittable for Sphere {
    // Implementation from ray tracing in a weekend
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction.len_squared();
        let half_b = ray.origin.dot(&ray.direction);
        let c = ray.origin.len_squared() - self.radius * self.radius;
        let disc = half_b*half_b - a*c;

        if disc < 0.0 {
            None
        } else {
            let disc_sqrt = disc.sqrt();

            let mut distance = -half_b - disc_sqrt;
            if distance <= NEAR_ZERO {
                distance = -half_b + disc_sqrt;
            }
            if distance <= NEAR_ZERO {
                return None;
            }

            distance /= a;
            let w = ray.at(distance);
            Some(Intersection::new(
                    self.norm_at(&w),
                    w,
                    ray,
                    distance,
                    ))
        }

    }

    /// Box containing the circle
    ///
    /// # Examples
    ///
    /// ```
    /// use rendering::core::Vector3f;
    /// use rendering::world::{Hittable, shapes::Sphere};
    ///
    /// let sph = Sphere::new(1.0);
    /// let b = sph.bounding_box();
    ///
    /// assert!(b.min.x == -1.0 && b.min.y == -1.0 && b.min.z == -1.0);
    /// assert!(b.max.x == 1.0 && b.max.y == 1.0 && b.max.z == 1.0);
    /// ```
    fn bounding_box(&self) -> Bound3f {
        let offset = Vector3f::new(self.radius);

        Bound3f::new(-offset, offset)
    }
}

impl Instancable for Sphere {}

impl Into<DynHittable> for Sphere {
    fn into(self) -> DynHittable {
        DynHittable::new(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_intersect() {
        let sph = Sphere::new(2.0);

        let ray = Ray {
            origin: Vector3f::new_xyz(1.0, 0.0, 0.0),
            direction: Vector3f::new_xyz(0.0, 1.0, 1.5).norm(),
        };

        let dist = sph.intersect(&ray).unwrap();
        assert!((dist.t - 1.732).abs() < 0.01);
    }
}
