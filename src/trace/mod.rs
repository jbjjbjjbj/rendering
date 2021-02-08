use crate::scene::Scene;
use crate::core::{Spectrum, Ray, Vector3f};
use crate::sample::Sampler;

mod pathtrace;
pub use pathtrace::PathTracer;

/// Simple surface normal tracer
///
/// This ray tracer bases color values on the hit surface normals
pub struct NormTracer<'a> {
    scn: &'a Scene,
}

/// Alias for chosen trace implementation.
///
/// This is swiched at compile time to save alot of time.
pub type DefaultTracer<'a> = PathTracer<'a>;

pub trait Tracer {
    fn trace(&self, sampler: &mut dyn Sampler, ray: &Ray) -> Spectrum;
}

impl NormTracer<'_> {
    pub fn new(scn: &Scene) -> NormTracer {
        NormTracer {scn}
    }
}

impl Tracer for NormTracer<'_> {
    fn trace(&self, _: &mut dyn Sampler, ray: &Ray) -> Spectrum {
        // Trace ray, we dont care about material
        if let Some(si) = self.scn.intersect(ray) {
            let norm = si.i.n * 0.5 + Vector3f::new(0.5);

            return Spectrum::new_rgb(norm.x, norm.y, norm.z);
        }

        Spectrum::new_rgb(0.0, 0.0, 0.0)
    }
}
