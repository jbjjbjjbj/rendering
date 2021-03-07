use super::Material;
use crate::core::{Ray, Spectrum};

pub struct SkyLight {
}

impl SkyLight {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for SkyLight {
    fn emitted(&self, ray: &Ray) -> Option<Spectrum> {
        let t = (ray.direction.norm().y + 1.0) * 0.5;

        Some(Spectrum::new_rgb(1.0, 1.0, 1.0) * (1.0-t) + Spectrum::new_rgb(0.5, 0.7, 1.0) * t)
    }
}
