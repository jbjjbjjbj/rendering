//! Used to represent color
//!
//! Currently only implements RGB colors
use crate::Float;
use std::ops;

// TODO implement SampledSpectrum instead for nicer images

#[derive(Clone, Default)]
pub struct Spectrum {
    c: [Float; 3],
}

impl Spectrum {
    pub fn new_rgb(r: Float, g: Float, b: Float) -> Spectrum {
        Spectrum { c: [r, g, b] }
    }

    pub fn to_rgb(&self, scale: Float) -> (Float, Float, Float) {
        (self.c[0] * scale, self.c[1] * scale, self.c[2] * scale)
    }
}

impl std::ops::Mul<Float> for &Spectrum {
    type Output = Spectrum;

    fn mul(self, op: Float) -> Self::Output {
        Self::Output::new_rgb(
            self.c[0] * op,
            self.c[1] * op,
            self.c[2] * op,
            )
    }
}

impl std::ops::AddAssign<&Self> for Spectrum {
    fn add_assign(&mut self, op: &Self) {
        self.c[0] += op.c[0];
        self.c[1] += op.c[1];
        self.c[2] += op.c[2];
    }
}
