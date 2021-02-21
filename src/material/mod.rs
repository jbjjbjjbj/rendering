use crate::core::{Ray, Spectrum};
use crate::world::Intersection;
use crate::sample::Sampler;

mod lambertian;
mod reflectant;

pub use lambertian::Lambertian;
pub use reflectant::Reflectant;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, i: &Intersection, sampler: &mut dyn Sampler) -> Option<(Spectrum, Ray)>;
}
