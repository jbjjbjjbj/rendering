//! Manages world objects, and implements intersection
pub mod shapes;

mod scene;
pub mod container;
mod hittable;
mod instancing;

pub use scene::*;
pub use hittable::{Intersection, Hittable, DynHittable};
pub use instancing::{Instance, Instancable};

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

impl Into<DynHittable> for Object {
    fn into(self) -> DynHittable {
        DynHittable::new(Box::new(self))
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
