use super::{Hittable, Intersection};
use crate::core::Ray;

pub struct HittableList {
    elems: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, h: Box<dyn Hittable>) {
        self.elems.push(h);
    }
}

impl Hittable for HittableList {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut min: Option<Intersection> = None;

        for e in self.elems.iter() {
            if let Some(i) = e.intersect(&ray) {
                match min {
                    // Do nothing if distance is bigger than min
                    Some(ref min_i) if min_i.t < i.t => {},
                    // If no existing min or closer than
                    _ => min = Some(i),
                }
            }
        }

        min
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            elems: Vec::new(),
        }
    }
}
