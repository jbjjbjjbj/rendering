use crate::core::{Ray, Intersection};
use crate::material::Material;

use super::Object;

pub struct Scene {
    objs: Vec<Object>,
}

pub struct SceneIntersect<'a> {
    pub mat: &'a dyn Material,
    pub i: Intersection,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objs.push(obj);
    }

    pub fn add_objects(&mut self, objs: Vec<Object>) {
        for obj in objs {
            self.add_object(obj);
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersect> {
        let mut min: Option<SceneIntersect> = None;

        for obj in self.objs.iter() {
            if let Some(i) = obj.shape.intersect(&ray) {
                match min {
                    Some(ref si) if si.i.t < i.t => (),
                    _ => min = Some(SceneIntersect {i, mat: obj.mat.as_ref() }),
                }
            }
        }

        min
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objs: Vec::new(),
        }
    }
}
