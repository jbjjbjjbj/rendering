//! Manages world objects, and implements intersection
pub mod shapes;

mod scene;
pub mod container;
mod hittable;
pub use scene::*;
pub use hittable::{Intersection, Hittable};

use std::sync::Arc;
use crate::material::Material;
use crate::core::{Bound3f, Ray};

pub struct Object {
    pub shape: Box<dyn Hittable + Sync>,
    pub mat: Arc<dyn Material + Sync + Send>,
}

impl Object {
    pub fn new(mat: Arc<dyn Material + Sync + Send>, shape: Box<dyn Hittable + Sync>) -> Self {
        Object {
            mat,
            shape,
        }
    }
}

impl Hittable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(mut inter) = self.shape.intersect(ray) {
            inter.add_material_if_none(self.mat.as_ref());
           Some(inter)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Bound3f {
        self.shape.bounding_box()
    }
}
