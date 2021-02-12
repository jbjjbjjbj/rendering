use crate::{M_PI, Float};
use crate::core::{Vector3f, Vector2f};

mod uniform;

pub use uniform::UniformSampler;

fn distribute_between(x: Float, a: Float, b: Float) -> Float {
    x * (b - a) + a
}

pub trait Sampler {
    fn get_sample(&mut self) -> Float;

    fn get_sample_2d(&mut self) -> Vector2f {
        Vector2f::new_xy(self.get_sample(), self.get_sample())
    }

    fn clone_and_seed(&mut self) -> Box<dyn Sampler + Send>;

    fn get_unit_vector(&mut self) -> Vector3f {
        let s2d = self.get_sample_2d();

        let lambda = distribute_between(s2d.x, -M_PI, M_PI);
        let costheta = 2.0 * s2d.y - 1.0;
        let sintheta = costheta.acos().sin();

        Vector3f::new_xyz(
            lambda.cos() * sintheta,
            lambda.sin() * sintheta,
            costheta,
            )
    }

    fn get_in_circle(&mut self) -> Vector2f {
        let s2d = self.get_sample_2d();

        let d = s2d.x.sqrt();
        let theta = s2d.y * 2.0 * M_PI;

        Vector2f::new_xy(
            d * theta.cos(),
            d * theta.sin(),
            )
    }
}
