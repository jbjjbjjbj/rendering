pub mod vector;
pub mod bound;
pub mod camera;
mod spectrum;

use std::ops::{Add, Sub, Mul};
use std::cmp;

pub trait Number:
    Copy +
    cmp::PartialOrd +
    Sub<Output = Self> +
    Add<Output = Self> +
    Mul<Output = Self>
{}

impl Number for usize {}
impl Number for i32 {}
impl Number for f32 {}

// Used throughout the program
type Float = f32;
