use crate::core::*;
use crate::Float;

#[derive(Clone)]
pub struct Pixel {
    rgb: Spectrum,
    samples: u32,
}

pub struct Film {
    size: Vector2i,
    drawingBound: Bound2i,

    pixels: Vec<Pixel>,
}

pub struct FilmTile {
    bounds: Bound2i,
    size: Vector2i,

    pixels: Vec<Pixel>,
}

//const HalfPixel = Vector2f::new(0.5);

impl Pixel {
    fn new() -> Pixel {
        Pixel {
            rgb: Default::default(),
            samples: 0,
        }
    }

    fn add(&mut self, c: &Spectrum, weight: Float) {
        self.rgb += &(c * weight);
        self.samples += 1;
    }
}

impl std::ops::AddAssign<&Self> for Pixel {
    fn add_assign(&mut self, op: &Self) {
        self.rgb += &op.rgb;
        self.samples += op.samples;
    }
}

impl Film {
    pub fn new(size: Vector2i) -> Film {
        let area = size.x * size.y;
        Film {
            size,
            drawingBound: Bound2i::new(&Vector2i::new(0), &size),
            pixels: vec![Pixel::new(); area as usize],
        }
    }

    pub fn get_tile(&self, bound: &Bound2i) -> FilmTile {
        FilmTile::new(
            bound,
        )

    }

    pub fn commit_tile(&mut self, tile: &FilmTile) {
        let offset = tile.bounds.min;

        for y in 0 ..= tile.size.y {
            let rowindex = (offset.y + y) * self.size.x;
            let prowindex = y * tile.size.x;

            for x in 0 ..= tile.size.x {
                let index = offset.x + x + rowindex;
                let pindex: i32 = x + prowindex;

                self.pixels[index as usize] += &tile.pixels[pindex as usize];
            }
        }

    }
}

impl FilmTile {
    fn new(bounds: &Bound2i) -> FilmTile {
        FilmTile {
            bounds: bounds.clone(),
            pixels: vec![Pixel::new(); bounds.area() as usize],
            size: bounds.diagonal(),
        }
    }

    pub fn add_sample(&mut self, point: &Vector2f, c: Spectrum) {
        let point = Vector2i::from(point.floor());
        // Subtract the offset
        let point = point - self.bounds.min;

        let index = point.x + point.y * self.size.x;

        let pixel = self.pixels.get_mut(index as usize).unwrap();
        pixel.add(&c, 1.0);
    }
}
