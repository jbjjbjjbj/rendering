//! Manages world objects, and implements intersection
pub mod shapes;

mod scene;
mod container;
mod hittable;
pub use scene::*;
pub use hittable::{Intersection, Hittable};
pub use container::HittableList;

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
        self.shape.intersect(ray).map(|mut i| {i.m = Some(self.mat.as_ref()); i})
    }

    fn bounding_box(&self) -> Option<Bound3f> {
        self.shape.bounding_box()
    }
}
