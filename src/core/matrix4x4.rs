use crate::{Number, Float};

pub struct Matrix4x4<T: Number> {
    pub m: [[T; 4]; 4],
}

pub type Matrix4x4f = Matrix4x4<Float>;

impl<T: Number> Matrix4x4<T> {
    pub fn new(v00: T, v01: T, v02: T, v03: T,
               v10: T, v11: T, v12: T, v13: T,
               v20: T, v21: T, v22: T, v23: T,
               v30: T, v31: T, v32: T, v33: T) -> Self {
        Self {
            m: [
                [v00, v01, v02, v03],
                [v10, v11, v12, v13],
                [v20, v21, v22, v23],
                [v30, v31, v32, v33],
            ],
        }
    }
}

impl Matrix4x4f {
    pub fn new_ident(v: Float) -> Self {
        let z = 0.0;
        Self::new(
            v, z, z, z,
            z, v, z, z,
            z, z, v, z,
            z, z, z, v,
            )
    }
}
