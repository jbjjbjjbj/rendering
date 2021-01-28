mod vector;
mod bound;
mod camera;
mod spectrum;

use std::ops::{Sub, Mul};
use std::cmp;

pub trait Number:
    Copy +
    cmp::PartialOrd +
    Sub<Output = Self> +
    Mul<Output = Self>
{}

impl Number for usize {}
impl Number for i32 {}
impl Number for f32 {}

// Used throughout the program
type Float = f32;
