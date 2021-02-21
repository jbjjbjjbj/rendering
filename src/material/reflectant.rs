use crate::Float;
use crate::core::{Ray, Spectrum, Vector3f};
use crate::world::Intersection;
use super::Material;
use crate::sample::Sampler;

pub struct Reflectant {
    color: Spectrum,
    fuzz: Option<Float>,
}

impl Reflectant {
    pub fn new(c: Spectrum, fuzz: Option<Float>) -> Reflectant {
        Reflectant {
            color: c,
            fuzz,
        }
    }
}

fn reflect(v: &Vector3f, n: &Vector3f) -> Vector3f {
    *v - *n * (2.0 * v.dot(n))
}

impl Material for Reflectant {
    fn scatter(&self, ray: &Ray, i: &Intersection, sampler: &mut dyn Sampler) -> Option<(Spectrum, Ray)> {
        // Find reflectance vector
        let mut reflected = reflect(&ray.direction, &i.n);
        if let Some(fuzz) = self.fuzz {
            reflected += &(sampler.get_unit_vector() * fuzz);
        }

        Some((
                self.color,
                Ray::new(i.p, reflected.norm()),
                ))
    }
}


