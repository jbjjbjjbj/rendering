use crate::{Number, Float};
use std::ops;

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

    pub fn initial(initial: T) -> Self {
        Self {
            m : [ [initial; 4]; 4]
        }
    }

    pub fn inverse(&self) -> Self {

    }
}

impl<T: Number> ops::Mul for &Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn mul(self, op: Self) -> Self::Output {
        let mut res = Self::Output::initial(Default::default());

        // Not very fast
        for i in 0..4 {
            for j in 0..4 {
                res.m[i][j] = self.m[i][0] * op.m[0][j] +
                    self.m[i][1] * op.m[1][j] +
                    self.m[i][2] * op.m[2][j] +
                    self.m[i][3] * op.m[3][j];
            }
        }

        res
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
