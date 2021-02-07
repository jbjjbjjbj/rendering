pub mod core;
pub mod camera;
pub mod render;
pub mod scene;
pub mod trace;
pub mod sample;
pub mod material;

use std::ops::{Add, Sub, Mul, DivAssign, AddAssign, Neg};
use std::cmp;
use std::fmt;
use std::f64::consts::PI;

/// Trait used to implement generics
///
/// This is used in Bound and Vectors
pub trait Number:
    Copy +
    cmp::PartialOrd +
    Sub<Output = Self> +
    Add<Output = Self> +
    Mul<Output = Self> +
    Neg<Output = Self> + 
    DivAssign +
    AddAssign +
    fmt::Display
{}

impl Number for i32 {}
impl Number for f32 {}
impl Number for f64 {}

/// Used for representing floating point values throughout the program
/// 
/// A higher precision type will require more ram
pub type Float = f64;

pub const M_PI: Float = PI;
pub const NEAR_ZERO: Float = 1e-8;
