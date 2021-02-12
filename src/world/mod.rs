//! Manages world objects, and implements intersection
pub mod shapes;

mod scene;
pub use scene::*;

use std::sync::Arc;
use crate::core::Hittable;
use crate::material::Material;

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
