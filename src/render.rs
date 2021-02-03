//! Implements the main render loop
//!
//! This is not a final design
use crate::camera::film::FilmTile;
use crate::camera::Camera;
use crate::scene::Scene;

use crate::core::{Vector2f, Spectrum};
use crate::Float;

const HALF_PIXEL: Vector2f = Vector2f {x: 0.5, y: 0.5 };

pub struct RenderTask {
    pub tile: Box<FilmTile>,
    samples: u32,
}

pub struct RenderContext<'a> {
    pub scn: &'a Scene,
    pub cam: &'a Camera,
}

impl RenderTask {
    pub fn new(tile: Box<FilmTile>, samples: u32) -> Self {
        Self { tile, samples }
    }

    fn render_at(&self, ctx: &RenderContext, x: i32, y: i32) -> Spectrum {
        // Create a ray
        let (r, _) = ctx.cam.generate_ray(&Vector2f::new_xy(x as Float, y as Float));

        // Trace ray
        if let Some(_) = ctx.scn.intersect(r) {
            return Spectrum::new_rgb(0.5, 0.5, 0.0);
        }

        Spectrum::new_rgb(0.0, 0.0, 0.0)

    }

    pub fn render(&mut self, ctx: &RenderContext) {
        let b = self.tile.bounds.clone();
        for x in b.min.x .. b.max.x {
            for y in b.min.y .. b.max.y {
                let p = Vector2f::new_xy(x as Float, y as Float);

                self.tile.add_sample(&p, self.render_at(ctx, x, y))
            }
        }
    }
}
