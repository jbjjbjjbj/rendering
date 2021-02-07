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

    pub fn trace_recur(&self, sampler: &mut dyn Sampler, ray: &Ray, depth: i32) -> Spectrum {

        if depth == 0 {
            return Spectrum::ZERO;
        }

        if let Some((mat, i)) = self.scn.intersect(ray) {
            if let Some((scalar, nray)) = mat.scatter(ray, &i, sampler) {
                return self.trace_recur(sampler, &nray, depth-1) * scalar;
            } else {
                return Spectrum::ZERO;
            }
        }

        // Simulates a sky
        let t = (ray.direction.norm().y + 1.0) * 0.5;
        Spectrum::new_rgb(1.0, 1.0, 1.0) * (1.0-t) + Spectrum::new_rgb(0.5, 0.7, 1.0) * t

    }
}

impl Tracer for PathTracer<'_> {
    fn trace(&self, sampler: &mut dyn Sampler, ray: &Ray) -> Spectrum {
        self.trace_recur(sampler, ray, self.depth)
    }
}
