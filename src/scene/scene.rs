use crate::core::{Ray, Intersection};
use crate::material::Material;

use super::Object;

pub struct Scene {
    objs: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objs.push(obj);
    }

    pub fn add_objects(&mut self, objs: Vec<Object>) {
        for obj in objs {
            self.add_object(obj);
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(&dyn Material, Intersection)> {
        for obj in self.objs.iter() {
            if let Some(i) = obj.shape.intersect(&ray) {
                return Some((obj.mat.as_ref(), i))
            }
        }

        None
    }
}

