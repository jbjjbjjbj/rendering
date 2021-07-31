use crate::world::{Object, Hittable, Intersection};
use crate::core::{Bound3f, Ray};


pub struct HittableList {
    elems: Vec<Object>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, h: Object) {
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

    fn bounding_box(&self) -> Bound3f {
        let mut bound: Bound3f = Bound3f::EMPTY;

        for e in self.elems.iter() {
            let eb = e.bounding_box();
            bound = bound.combine(&eb);
        }

        bound
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            elems: Vec::new(),
        }
    }
}

