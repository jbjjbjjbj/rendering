use crate::vector::{Vector2, Vector2f};
use crate::Float;
use crate::bounding::Bound2i;
use super::filter::Filter;

#[derive(Clone)]
pub struct Pixel {
    pub rgb: [Float; 3],
}

pub struct Film<'a> {
    pub resolution: Vector2<usize>,

    pixels: Vec<Pixel>,
    filter: &'a dyn Filter,
    filter_radius: Vector2f,
}

pub struct FilmTile {
    
}

impl Pixel {
    fn new() -> Pixel {
        Pixel { rgb: [0.0, 0.0, 0.0] }
    }
}

impl Film<'_> {
    fn new(resolution: Vector2<usize>, filter: & dyn Filter) -> Film {
        let area = resolution.x * resolution.y;
        Film {
            resolution,
            pixels: vec![Pixel::new(); area],
            filter,
            filter_radius: filter.radius()
        }
    }

    fn get_tile(bound: &Bound2i) {
        // Used to calculate descrete coordinates into continues
        let halfpixel = Vector2f::from_xy(0.5, 0.5);
    }

    fn splat(&mut self, point: &Vector2<usize>, pixel: &Pixel) {
        let index = point.x + point.y * self.resolution.x;

        self.pixels[index] = pixel.clone();
    }
}

