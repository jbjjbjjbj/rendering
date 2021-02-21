use crate::core::Ray;

use super::{Object, HittableList, Hittable, Intersection};

pub struct Scene {
    content: HittableList,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, obj: Object) {
        self.content.add(Box::new(obj));
    }

    pub fn add_objects(&mut self, objs: Vec<Object>) {
        for obj in objs {
            self.add_object(obj);
        }
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.content.intersect(ray)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            content: HittableList::new(),
        }
    }
}
