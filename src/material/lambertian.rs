use super::Material;
use crate::core::{Intersection, Ray, Spectrum};
use crate::sample::Sampler;

use std::rc::Rc;

pub struct Lambertian {
    color: Spectrum,
}

impl Lambertian {
    pub fn new(c: Spectrum) -> Lambertian {
        Lambertian {
            color: c,
        }
    }

    pub fn new_rc(c: Spectrum) -> Rc<dyn Material> {
        Rc::new(Self::new(c))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, i: &Intersection, sampler: &mut dyn Sampler) -> Option<(Spectrum, Ray)> {
        let mut newray = Ray {
            origin: i.p,
            direction: i.n + sampler.get_unit_vector(),
        };

        // Make sure that the resulting direction is not (0, 0, 0)
        if newray.direction.near_zero() {
            newray.direction = i.n;
        }

        Some((self.color, newray))
    }
}
