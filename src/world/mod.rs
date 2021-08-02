//! Manages world objects, and implements intersection
pub mod shapes;

mod scene;
pub mod container;
mod hittable;
pub use scene::*;
pub use hittable::{Intersection, Hittable, DynHittable};
pub use shapes::Shape;

use std::sync::Arc;
use crate::material::Material;
use crate::core::{Bound3f, Ray};

pub struct Object {
    pub inner: DynHittable,
    pub mat: Arc<dyn Material>,
}

impl Object {
    pub fn new<T: Into<DynHittable>>(mat: Arc<dyn Material>, inner: T) -> Self {
        Object {
            mat,
            inner: inner.into(),
        }
    }
}

impl Hittable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(mut inter) = self.inner.intersect(ray) {
            inter.add_material_if_none(self.mat.as_ref());
            Some(inter)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Bound3f {
        self.inner.bounding_box()
    }
}
