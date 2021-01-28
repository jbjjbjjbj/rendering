use crate::vector::{Vector2, Vector2f};
use crate::Float;
use crate::bound;
use bound::{Bound2i, Bound2f};
use super::filter::Filter;

#[derive(Clone)]
pub struct Pixel {
    pub rgb: [Float; 3],
}

pub struct Film<'a> {
    pub resolution: Vector2<usize>,

    pixels: Vec<Pixel>,
    filter: &'a Filter,
}

pub struct FilmTile {
    
}

impl Pixel {
    fn new() -> Pixel {
        Pixel { rgb: [0.0, 0.0, 0.0] }
    }
}

impl Film<'_> {
    fn new(resolution: Vector2<usize>, filter: & Filter) -> Film {
        let area = resolution.x * resolution.y;
        Film {
            resolution,
            pixels: vec![Pixel::new(); area],
            filter,
        }
    }

    fn get_tile(bound: &Bound2i) {
        // Used to calculate descrete coordinates into continues
        let halfpixel = Vector2f::from_xy(0.5, 0.5);
        let fbound = Bound2f::from(bound);



        //let tilebound = bound::intersect(bound, 
    }

    fn splat(&mut self, point: &Vector2<usize>, pixel: &Pixel) {
        let index = point.x + point.y * self.resolution.x;

        self.pixels[index] = pixel.clone();
    }
}

