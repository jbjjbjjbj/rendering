//! Generates rays from screen coordinates
//!
//! Generates rays in world space from screen coordinates.
//! Future versions should also simulate depth of field.
//!
//! # Examples
//!
//! ```
//! use rendering::camera::{CameraSettings, Camera};
//! use rendering::core::{Vector3f, Vector2f, Vector2i};
//! use rendering::sample::UniformSampler;
//!
//! let set = CameraSettings {
//!     origin: Vector3f::new(10.0),
//!     target: Vector3f::new(0.0),
//!     up: Vector3f::new_xyz(0.0, 1.0, 0.0),
//!     fov: 90.0, 
//!     filmsize: Vector2i::new(10),
//!     focus: None,
//!     aperture: None,
//! };
//!
//! let cam = Camera::new(&set);
//! let mut sampler = UniformSampler::new();
//!
//! let (r, _) = cam.generate_ray(&Vector2f::new(5.0), &mut sampler);
//! let dir = r.direction;
//!
//! assert!(
//!     dir.x == -0.6031558065478413 &&
//!     dir.y == -0.6599739684616743 &&
//!     dir.z == -0.4479257014065748
//!     );
//!
//! ```
use crate::Float;
use crate::core::{Vector3f, Vector2f, Vector2i, Ray};
use crate::sample::Sampler;

/// A simple perspective camera
pub struct Camera {
    /// The camera origin in the screen
    origin: Vector3f,
    /// Vector from camera origin to the screen lower left corner of the film plane
    screen_origin: Vector3f,
    /// Scaling vectors from screen_origin
    qx: Vector3f,
    qy: Vector3f,

    /// Value for depth of view
    lens_radius: Option<Float>,
}

/// Settings for initializing camera
pub struct CameraSettings {
    /// Where rays originate from
    pub origin: Vector3f,
    /// Point where center of image is pointed at
    pub target: Vector3f,
    /// Vector that will be up in the resulting image
    pub up: Vector3f,
    /// The vertical field of view in degrees.
    /// Currently must be between [0; 180[.
    pub fov: Float,
    /// The film aspect ratio, height / width
    pub filmsize: Vector2i,
    /// The lens aperture
    ///
    /// Depth of view is disabled if None
    pub aperture: Option<Float>,
    /// The distance to the focus plane
    ///
    /// if None it will be set to the distance between origin and target
    pub focus: Option<Float>,
}

impl Camera {
    /// Create a new camera look at a target
    pub fn new(set: &CameraSettings) -> Camera {
        let filmsize = Vector2f::from(set.filmsize);
        // Calculate translation vectors
        let mut forward = set.target - set.origin;

        let focus = set.focus.unwrap_or_else(|| forward.length());

        forward.norm_in();

        let right = set.up.cross(&forward).norm();
        let newup = forward.cross(&right).norm();

        let aspect = (filmsize.y) / (filmsize.x);
        // Calculate screen size from fov and focus distance
        let width = 2.0 * focus * (set.fov / 2.0).to_radians().tan();
        let height = aspect * width;

        // Calculate screen scaling vectors
        let qx = right * (width / (filmsize.x - 1.0));
        let qy = newup * (height / (filmsize.y - 1.0));

        let screen_origin = forward * focus - (right * (width/2.0)) + (newup * (height/2.0));

        Camera {
            origin: set.origin,
            screen_origin,
            qx,
            qy,
            lens_radius: set.aperture.map(|a| a / 2.0),
        }
    }

    /// Generates a ray a screen space point
    ///
    /// The point coordinates should be between [0,1) with (0, 0) being the upper left corner
    ///
    /// Will return a ray and a weight
    ///
    /// The direction of the returned way is normalized
    pub fn generate_ray(&self, point: &Vector2f, sampler: &mut dyn Sampler) -> (Ray, Float) {
        // Depth of view origin offset
        let ooffset = match self.lens_radius {
            Some(r) => {
                let rand_dir = sampler.get_in_circle() * r;
                self.qx * rand_dir.x + self.qy * rand_dir.y
            },
            None => Vector3f::ZERO,
        };

        let dir = self.screen_origin + (self.qx * point.x) - (self.qy * point.y) - ooffset;

        (
            Ray { origin: self.origin + ooffset, direction: dir.norm() },
            1.0
            )
    }
}
