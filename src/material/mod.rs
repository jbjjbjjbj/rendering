use crate::core::{Ray, Spectrum};
use crate::world::Intersection;
use crate::sample::Sampler;

mod lambertian;
mod diffuse_light;
mod sky_light;
mod dielectric;

pub use lambertian::Lambertian;
pub use diffuse_light::DiffuseLight;
pub use sky_light::SkyLight;
pub use dielectric::Dielectric;

pub trait Material: Sync + Send {
    fn scatter(&self, _: &Ray, _: &Intersection, _: &mut dyn Sampler) -> Option<(Spectrum, Ray)> {
        None
    }

    fn emitted(&self, _: &Ray) -> Option<Spectrum> {
        None
    }
}
