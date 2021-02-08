//! Implements the main render loop
//!
//! This is not a final design
use crate::camera::film::FilmTile;
use crate::camera::Camera;
use crate::trace::{DefaultTracer, Tracer};
use crate::sample::Sampler;

use crate::core::{Vector2f};
use crate::Float;

pub struct RenderTask {
    pub tile: Box<FilmTile>,
    samples: u32,
}

pub struct RenderContext<'a> {
    pub cam: &'a Camera,
    pub trc: &'a DefaultTracer<'a>,
}

impl RenderTask {
    pub fn new(tile: Box<FilmTile>, samples: u32) -> Self {
        Self { tile, samples }
    }

    fn render_at(&mut self, ctx: &RenderContext, x: i32, y: i32, sampler: &mut dyn Sampler) {
        let corner = Vector2f::new_xy(x as Float, y as Float);

        for _ in 0..self.samples {
            let p = corner + sampler.get_sample_2d();

            // Create a ray
            let (r, _) = ctx.cam.generate_ray(&p, sampler);

            self.tile.add_sample(&p, ctx.trc.trace(sampler, &r));
        }
    }

    pub fn render(&mut self, ctx: &RenderContext, sampler: &mut dyn Sampler) {
        let b = self.tile.bounds.clone();
        for y in b.min.y .. b.max.y {
            for x in b.min.x .. b.max.x {
                self.render_at(ctx, x, y, sampler);
            }
        }
    }
}
