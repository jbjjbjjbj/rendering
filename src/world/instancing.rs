use crate::Float;
use crate::core::{Ray, Vector3f, Bound3f};
use crate::world::{Hittable, DynHittable, Intersection};

pub struct Instance {
    inner: DynHittable,
    translate: Vector3f,
}

pub trait Instancable: Into<DynHittable> {
    fn translate(self, x: Float, y: Float, z: Float) -> Instance {
        self.translate_vec(Vector3f::new_xyz(x, y, z))
    }

    fn translate_vec(self, offset: Vector3f) -> Instance {
        Instance::new_translate(self.into(), offset)
    }
}

impl Instance {
    pub fn new_translate(inner: DynHittable, offset: Vector3f) -> Self {
        Self {
            inner,
            translate: offset,
        }
    }
}

impl Hittable for Instance {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let ray = Ray::new(ray.origin - self.translate, ray.direction);

        self.inner.intersect(&ray).map(|mut int| {
            int.p += &self.translate;
            int
        })
    }

    fn bounding_box(&self) -> Bound3f {
        self.inner.bounding_box().offset(self.translate)
    }
}

impl Into<DynHittable> for Instance {
    fn into(self) -> DynHittable {
        DynHittable::new(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::shapes::Sphere;

    #[test]
    fn sphere_translation() {
        //let sph = Sphere::new(2.0).translate(2.0, 3.0, 4.0);
        let sph = Sphere::new(2.0).translate(2.0, 3.0, 4.0);

        let ray = Ray {
            origin: Vector3f::new_xyz(1.0, 0.0, 0.0),
            direction: Vector3f::new_xyz(0.0, 1.0, 1.5).norm(),
        };

        let dist = sph.intersect(&ray).unwrap();
        println!("hey {}", dist.t);
        assert!((dist.t - 3.28).abs() < 0.01);
    }
}
