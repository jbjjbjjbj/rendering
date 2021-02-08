//! Implements sphere
//!
//! Spheres are relatively easy to calculate intersections between
use crate::Float;
use crate::core::{Ray, Vector3f, Hittable, Intersection};

pub struct Sphere {
    radius: Float,
    center: Vector3f,
}

impl Sphere {
    pub fn new(radius: Float, center: Vector3f) -> Sphere {
        Sphere {
            radius,
            center,
        }
    }

    fn norm_at(&self, point: &Vector3f) -> Vector3f {
        let mut v = *point - self.center;
        v /= self.radius;
        v
    }
}

impl Hittable for Sphere {
    // Implementation from ray tracing in a weekend
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let disc = half_b*half_b - a*c;

        if disc < 0.0 {
            return None
        } else {
            let distance = (-half_b - disc.sqrt()) / a;
            if distance < 0.0 {
                return None
            }
            let w = ray.at(distance);
            Some(Intersection {
                n: self.norm_at(&w),
                p: w,
                t: distance,
            })
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_intersect() {
        let sph = Sphere::new(2.0, Vector3f::new_xyz(2.0, 3.0, 4.0));

        let ray = Ray {
            origin: Vector3f::new_xyz(1.0, 0.0, 0.0),
            direction: Vector3f::new_xyz(0.0, 1.0, 1.5).norm(),
        };

        let dist = sph.intersect(&ray).unwrap();
        assert!((dist.t - 3.28).abs() < 0.01);
    }
}
