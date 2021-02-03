//! Implements how light is captured on film and how rays are generated
//!
//! The Film module specifies how calculated spectrum values contribute to the final image.
//!
//! The Camera class generated rays that can be cast into the scene.
//! This requires converting the film coordinates into real coordinates

pub mod film;
//pub mod filter;
pub mod camera;
