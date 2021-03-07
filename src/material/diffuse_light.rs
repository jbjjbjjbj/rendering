use super::Material;
use crate::core::{Ray, Spectrum};
use crate::Float;

pub struct DiffuseLight {
    color: Spectrum,
}

impl DiffuseLight {
    pub fn new(c: Spectrum) -> Self {
        Self {
            color: c,
        }
    }

    pub fn new_white(s: Float) -> Self {
        Self {
            color: Spectrum::new_rgb(s, s, s),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _: &Ray) -> Option<Spectrum> {
        Some(self.color)
    }
}
