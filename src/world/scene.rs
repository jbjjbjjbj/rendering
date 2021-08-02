use crate::core::{Bound3f, Ray};

use std::iter::IntoIterator;

use super::{Object, container, Hittable, Intersection};

type Container = container::HittableList;

pub struct Scene {
    content: Container,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, obj: Object) {
        self.content.add(obj);
    }

    pub fn add_objects<T>(&mut self, objs: T) 
    where
        T: IntoIterator<Item = Object>,
    {
        for obj in objs.into_iter() {
            self.add_object(obj);
        }
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.content.intersect(ray)
    }

    fn bounding_box(&self) -> Bound3f {
        self.content.bounding_box()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            content: Container::new(),
        }
    }
}
