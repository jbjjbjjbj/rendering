use super::Material;
use crate::core::{min, Vector3f, Spectrum, Ray};
use crate::world::Intersection;
use crate::sample::Sampler;
use crate::Float;
use crate::material::reflectant::reflect;

pub struct Dielectric {
    ratio: Float,
}

// Implementation from RTIOW
fn refract(v: Vector3f, n: Vector3f, r_ratio: Float, cos_theta: Float) -> Vector3f {
    let r_perp = (v + n * cos_theta) * r_ratio;
    let r_parallel = n * (-(1.0 - r_perp.len_squared()).abs().sqrt());

    r_perp + r_parallel
}

// Schlick Approximation, explained in RTIOW
fn fresnel(cos: Float, ratio: Float) -> Float {
    let mut r0 = (1.0-ratio) / (1.0+ratio);
    r0 = r0 * r0;

    r0 + (1.0-r0)*(1.0-cos).powi(5)
}

impl Dielectric {
    pub fn new(ratio: Float) -> Self {
        Self { ratio }
    }
}

impl Material for Dielectric {
    // Implementation from RTIOW
    fn scatter(&self, ray: &Ray, i: &Intersection, sampler: &mut dyn Sampler) -> Option<(Spectrum, Ray)> {
        let ratio = if i.front {1.0/self.ratio} else {self.ratio};

        let ray_dir = ray.direction.norm();
        let cos_theta = min((-ray_dir).dot(&i.n), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        // Test if it is possible for the ray the retract or if it must reflect.
        let cannot_refract = (ratio * sin_theta) > 1.0;
        let direction = if cannot_refract || (fresnel(cos_theta, ratio) > sampler.get_sample()) {
            reflect(ray_dir, i.n)
        } else {
            refract(ray_dir, i.n, ratio, cos_theta)
        };

        Some((
                Spectrum::WHITE,
                Ray::new(i.p, direction),
             ))
    }
}
