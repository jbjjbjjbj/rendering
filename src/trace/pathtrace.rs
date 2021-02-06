use crate::scene::Scene;
use crate::core::{Ray, Spectrum};
use crate::sample::Sampler;
use super::Tracer;

pub struct PathTracer<'a> {
    depth: i32,
    scn: &'a Scene,
}

impl PathTracer<'_> {
    pub fn new(scn: &Scene, depth: Option<i32>) -> PathTracer {
        let depth = depth.unwrap_or(-1);

        PathTracer {
            depth,
            scn,
        }
    }

    pub fn trace_recur(&self, sampler: &mut dyn Sampler, ray: &Ray) -> Spectrum {

        if let Some(i) = self.scn.intersect(ray) {
            // Get a random direction in the hemisphere a i.p
            // This is Lambertian reflection
            let target = i.p + i.n + sampler.get_unit_vector();
            return self.trace_recur(sampler, &Ray::new_to(i.p, target)) * 0.5;
        }

        // Simulates a sky
        let t = (ray.direction.norm().y + 1.0) * 0.5;
        Spectrum::new_rgb(1.0, 1.0, 1.0) * (1.0-t) + Spectrum::new_rgb(0.5, 0.7, 1.0) * t

    }
}

impl Tracer for PathTracer<'_> {
    fn trace(&self, sampler: &mut dyn Sampler, ray: &Ray) -> Spectrum {
        self.trace_recur(sampler, ray)
    }
}
