//! Defines the scene type which contains all the objects in the scene.
//! 
//! Also handles finding intersections between rays and shapes
pub mod shapes;

mod scene;
pub use scene::*;

use std::rc::Rc;
use crate::core::Hittable;
use crate::material::Material;

pub struct Object {
    pub shape: Box<dyn Hittable>,
    pub mat: Rc<dyn Material>,
}

impl Object {
    pub fn new(mat: Rc<dyn Material>, shape: Box<dyn Hittable>) -> Self {
        Object {
            mat,
            shape,
        }
    }
}
