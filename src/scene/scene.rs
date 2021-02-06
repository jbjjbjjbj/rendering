use super::{Intersection, Hittable};
use crate::core::Ray;

type Shape = Box<dyn Hittable>;

pub struct Scene {
    shps: Vec<Shape>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shps: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shp: Shape) {
        self.shps.push(shp);
    }

    pub fn add_shapes(&mut self, shps: Vec<Shape>) {
        for shp in shps {
            self.add_shape(shp);
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        for shp in self.shps.iter() {
            if let Some(i) = shp.intersect(&ray) {
                return Some(i)
            }
        }

        None
    }
}

