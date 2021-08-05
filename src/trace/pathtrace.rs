use crate::world::{Hittable, Scene};
use crate::core::{Ray, Spectrum};
use crate::material::Material;
use crate::sample::Sampler;
use super::Tracer;

pub struct PathTracer<'a> {
    depth: i32,
    scn: &'a Scene,
    background: Option<Box<dyn Material>>,
}

impl PathTracer<'_> {
    pub fn new(scn: &Scene, depth: Option<i32>, background: Option<Box<dyn Material>>) -> PathTracer {
        let depth = depth.unwrap_or(-1);

        PathTracer {
            depth,
            scn,
            background,
        }
    }

    pub fn trace_recur(&self, sampler: &mut dyn Sampler, ray: &Ray, depth: i32) -> Spectrum {

        if depth == 0 {
            return Spectrum::ZERO;
        }

        if let Some(i) = self.scn.intersect(ray) {
            if let Some(mat) = i.m {
                let mut col = Spectrum::ZERO;

                if let Some((scalar, nray)) = mat.scatter(ray, &i, sampler) {
                    col += &(self.trace_recur(sampler, &nray, depth-1) * scalar);
                }

                if let Some(c) = mat.emitted(ray) {
                    col += &c;
                }

                return col;
            }
        }

        // If no color return background
        if let Some(back) = &self.background {
            back.emitted(ray).unwrap_or(Spectrum::ZERO)
        } else {
            Spectrum::ZERO
        }
    }
}

impl Tracer for PathTracer<'_> {
    fn trace(&self, sampler: &mut dyn Sampler, ray: &Ray) -> Spectrum {
        self.trace_recur(sampler, ray, self.depth)
    }
}
