use crate::core::{Ray, Intersection, Spectrum};
use crate::sample::Sampler;

mod lambertian;

pub use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, ray: &Ray, i: &Intersection, sampler: &mut dyn Sampler) -> Option<(Spectrum, Ray)>;
}
