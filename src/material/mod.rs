use crate::core::{Ray, Intersection, Spectrum};
use crate::sample::Sampler;

mod lambertian;
mod reflectant;

pub use lambertian::Lambertian;
pub use reflectant::Reflectant;

pub trait Material {
    fn scatter(&self, ray: &Ray, i: &Intersection, sampler: &mut dyn Sampler) -> Option<(Spectrum, Ray)>;
}
