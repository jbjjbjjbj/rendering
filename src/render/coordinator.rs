//! Split a image into small tiles
//!
//! This enables multithreaded rendering.

use super::{RenderTask, RenderContext};
use crate::camera::Film;
use crate::core::{Bound2i, Vector2i};
use crate::sample::Sampler;

use std::sync::{Mutex};

struct Tiler {
    tilesize: Vector2i,

    fullbound: Bound2i,
    tilemap_size: Vector2i,
    tilemap_area: i32,

    next_tile: i32,
    tiles_done: i32,
}

pub struct RenderCoord<'a> {
    film: Mutex<&'a mut Film>,
    samples: u32,
    tiler: Mutex<Tiler>,
}

impl Tiler {
    pub fn new(fullsize: Vector2i, tilesize: Vector2i) -> Tiler {
        assert!(tilesize.x != 0 && tilesize.y != 0);

        let tilemap_size = fullsize / tilesize;

        Self {
            tilesize,

            fullbound: Bound2i::new(&Vector2i::ZERO, &fullsize),
            tilemap_size,
            tilemap_area: (tilemap_size.x * tilemap_size.y),

            next_tile: 0,
            tiles_done: 0,
        }

    }

    pub fn next_tile(&mut self) -> Option<(Bound2i, i32)> {
        // Check if we are outside
        if self.next_tile >= self.tilemap_area {
            return None;
        }

        // Convert the tile to xy in tilemap
        let tile = Vector2i::new_xy(self.next_tile / self.tilemap_size.x, self.next_tile % self.tilemap_size.x);
        let tile_index = self.next_tile;

        self.next_tile += 1;

        // Create a bound from the tilecoordinate
        let tile_start = tile * self.tilesize;
        // We need to make sure the resulting tile is inside the image bound
        Some((
                Bound2i::new(&tile_start, &(tile_start + self.tilesize)).intersect(&self.fullbound),
                tile_index,
                ))
    }

    /// Mark a index as done and return the overall process
    pub fn mark_done(&mut self, _index: i32) -> f32 {
        self.tiles_done += 1;

        println!("Progress: {}/{}", self.tiles_done, self.tilemap_area);
        self.tiles_done as f32 / self.tilemap_area as f32
    }
}

impl<'a> RenderCoord<'a> {
    pub fn new(film: &'a mut Film, tilesize: Vector2i, samples: u32) -> Self {
        let size = film.size;

        Self {
            film: Mutex::new(film),
            samples,
            tiler: Mutex::new(Tiler::new(size, tilesize)),
        }
    }

    pub fn next_task(&self) -> Option<RenderTask> {
        let (tile_bound, index) = self.tiler.lock().unwrap().next_tile()?;

        let film_tile = {
            let film = self.film.lock().unwrap();
            Box::new(film.get_tile(&tile_bound))
        };

        Some(RenderTask::new(film_tile, self.samples, index))
    }

    pub fn finish_task(&self, task: &RenderTask) {
        {
            let mut film = self.film.lock().unwrap();
            film.commit_tile(task.tile.as_ref());
        }

        self.tiler.lock().unwrap().mark_done(task.tile_index);
    }

    pub fn work(&self, ctx: &RenderContext, sampler: &mut dyn Sampler) {
        while let Some(mut task) = self.next_task() {
            task.render(ctx, sampler);
            self.finish_task(&task);
        }
    }

    pub fn run_threaded(&self, ctx: &RenderContext, sampler: &mut (dyn Sampler + Send), threads: u32) {
        crossbeam::scope(|scope| {
            for _ in 0..threads {
                let mut sampler = sampler.clone_and_seed();
                scope.spawn(move |_| {
                    self.work(ctx, sampler.as_mut());
                });
            }
        }).unwrap();
    }
}
