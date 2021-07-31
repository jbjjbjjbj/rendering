//! Used to represent color
//!
//! Currently only implements RGB colors
use crate::Float;

// TODO implement SampledSpectrum instead for nicer images

#[derive(Clone, Copy, Default)]
pub struct Spectrum {
    c: [Float; 3],
}

impl Spectrum {
    pub const ZERO: Self = Spectrum { c: [0.0; 3] };
    pub const WHITE: Self = Spectrum { c: [1.0; 3] };

    pub fn new_rgb(r: Float, g: Float, b: Float) -> Spectrum {
        Spectrum { c: [r, g, b] }
    }

    pub fn to_rgb(&self, scale: Float) -> (Float, Float, Float) {
        (self.c[0] * scale, self.c[1] * scale, self.c[2] * scale)
    }

    pub fn gamma_correct(&self) -> Self {
        Self::new_rgb(
            self.c[0].sqrt(),
            self.c[1].sqrt(),
            self.c[2].sqrt(),
            )
    }
}

impl std::ops::Mul<Float> for Spectrum {
    type Output = Spectrum;

    fn mul(self, op: Float) -> Self::Output {
        Self::Output::new_rgb(
            self.c[0] * op,
            self.c[1] * op,
            self.c[2] * op,
            )
    }
}

impl std::ops::Mul for Spectrum {
    type Output = Spectrum;

    fn mul(self, op: Self) -> Self::Output {
        Self::Output::new_rgb(
            self.c[0] * op.c[0],
            self.c[1] * op.c[1],
            self.c[2] * op.c[2],
            )
    }
}

impl std::ops::Div<Float> for Spectrum {
    type Output = Spectrum;

    fn div(self, op: Float) -> Self::Output {
        Self::Output::new_rgb(
            self.c[0] / op,
            self.c[1] / op,
            self.c[2] / op,
            )
    }
}

impl std::ops::Add for Spectrum {
    type Output = Spectrum;

    fn add(self, op: Self) -> Self::Output {
        Self::Output::new_rgb(
            self.c[0] + op.c[0],
            self.c[1] + op.c[1],
            self.c[2] + op.c[2],
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
