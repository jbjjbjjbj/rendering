mod sphere;
mod rectangle;

pub use sphere::Sphere;
pub use rectangle::{Rect, Plane};

use crate::world::{Hittable, Intersection};
use crate::core::{Bound3f, Ray};

pub enum Shape {
    Sphere(Sphere),
}

impl Hittable for Shape {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Self::Sphere(sph) => sph.intersect(ray)
        }
    }

    fn bounding_box(&self) -> Bound3f {
        match self {
            Self::Sphere(sph) => sph.bounding_box()
        }
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
    }
}
