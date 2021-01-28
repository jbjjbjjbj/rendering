use crate::vector::*;
use crate::Float;
use crate::bound;
use crate::spectrum::Spectrum;

use bound::{Bound2i, Bound2f};
use super::filter::Filter;
use std::rc::Rc;

#[derive(Clone)]
pub struct Pixel {
    pub rgb: [Float; 3],
}

pub struct Film {
    pub resolution: Vector2i,
    drawingBound: Bound2i,

    pixels: Vec<Pixel>,
    filter: Rc<Filter>,
}

pub struct FilmTile {
    pub bounds: Bound2i,
    filter: Rc<Filter>,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel { rgb: [0.0, 0.0, 0.0] }
    }
}

impl Film {
    pub fn new(resolution: Vector2i, filter: Rc<Filter>) -> Film {
        let area = resolution.x * resolution.y;
        Film {
            resolution,
            drawingBound: Bound2i::new(&Vector2::new(0), &resolution),
            pixels: vec![Pixel::new(); area as usize],
            filter,
        }
    }

    pub fn get_tile(&self, bound: &Bound2i) -> FilmTile {
        // Used to calculate descrete coordinates into continues
        let halfpixel = Vector2f::new_xy(0.5, 0.5);
        let fbound = Bound2f::from(bound);

        let p0 = Vector2i::from((fbound.min - halfpixel - self.filter.radius).ceil());
        let p1 = Vector2i::from((fbound.min - halfpixel + self.filter.radius).floor());

        let tilebound = bound::intersect(&Bound2i::new(&p0, &p1), &self.drawingBound);

        FilmTile { 
            bounds: tilebound,
            filter: self.filter.clone(),
        }

    }
}

impl FilmTile {
    fn add_sample(point: &Vector2f, c: Spectrum) {
        
    }
}
