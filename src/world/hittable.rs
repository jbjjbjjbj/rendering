use crate::core::{Vector3f, Bound3f, Ray};
use crate::Float;
use crate::material::Material;
use crate::world::Instancable;

use std::ops::Deref;

/// Returns the context of a intersection
pub struct Intersection<'a> {
    /// Normal vector at intersection
    pub n: Vector3f,
    pub p: Vector3f,
    pub front: bool,
    pub t: Float,
    pub m: Option<&'a dyn Material>,
}

impl<'a> Intersection<'a> {
    pub fn new(out_normal: Vector3f, point: Vector3f, ray: &Ray, t: Float) -> Self {
        let front = ray.direction.dot(&out_normal) < 0.0;
        Intersection {
            n: { if front { out_normal } else { -out_normal } },
            front,
            p: point,
            m: None,
            t,
        }
    }

    pub fn add_material_if_none(&mut self, mat: &'a dyn Material) {
        if let None = self.m {
            self.m = Some(mat);
        }
    }
}

/// Defines a common trait for objects in the scene
pub trait Hittable: Sync + Send {
    /// Returns the intersection with ray
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;

    /// Returns the axis alligned bounding box containing self
    fn bounding_box(&self) -> Bound3f;
}

pub struct DynHittable(Box<dyn Hittable>);

impl DynHittable {
    pub fn new(hit: Box<dyn Hittable>) -> Self {
        Self (hit)
    }
}

impl Deref for DynHittable {
    type Target = dyn Hittable;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl Instancable for DynHittable {}
