use crate::scene::Scene;
use crate::core::{Spectrum, Ray, Vector3f};

/// Simple surface normal tracer
///
/// This ray tracer bases color values on the hit surface normals
pub struct NormTracer {}

/// Alias for chosen trace implementation.
///
/// This is swiched at compile time to save alot of time.
pub type Tracer = NormTracer;

impl NormTracer {
    pub fn new() -> NormTracer {
        NormTracer {}
    }

    pub fn trace(&self, scn: &Scene, ray: &Ray) -> Spectrum {
        // Trace ray
        if let Some(i) = scn.intersect(ray) {
            let norm = i.n * 0.5 + Vector3f::new(0.5);

            return Spectrum::new_rgb(norm.x, norm.y, norm.z);
        }

        Spectrum::new_rgb(0.0, 0.0, 0.0)
    }
}
