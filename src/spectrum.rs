use crate::Float;

// TODO implement SampledSpectrum instead for nicer images

pub struct Spectrum {
    c: [Float; 3],
}

impl Spectrum {
    fn from_rgb(r: Float, g: Float, b: Float) -> Spectrum {
        Spectrum { c: [r, g, b] }
    }
}
