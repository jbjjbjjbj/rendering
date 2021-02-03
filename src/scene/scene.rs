use super::shapes::Shape;
use crate::Float;
use crate::core::Ray;

pub struct Scene {
    shps: Vec<Box<dyn Shape>>,
}

pub struct Intersection<'a> {
    pub shp: &'a dyn Shape,
    pub t: Float,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shps: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shp: Box<dyn Shape>) {
        self.shps.push(shp);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        for shp in self.shps.iter() {
            if let Some(t) = shp.intersect(&ray) {
                return Some(Intersection {
                    shp: shp.as_ref(),
                    t,
                })
            }
        }

        None
    }
}

