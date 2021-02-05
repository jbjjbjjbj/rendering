pub mod core;
pub mod camera;
pub mod render;
pub mod scene;
pub mod trace;
pub mod sample;

use std::ops::{Add, Sub, Mul, DivAssign};
use std::cmp;
use std::fmt;

/// Trait used to implement generics
///
/// This is used in Bound and Vectors
pub trait Number:
    Copy +
    cmp::PartialOrd +
    Sub<Output = Self> +
    Add<Output = Self> +
    Mul<Output = Self> +
    DivAssign +
    fmt::Display
{}

impl Number for i32 {}
impl Number for f32 {}

/// Used for representing floating point values throughout the program
/// 
/// A higher precision type will require more ram
pub type Float = f32;
