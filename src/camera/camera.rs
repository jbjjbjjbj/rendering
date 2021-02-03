//! Generates rays from screen coordinates
//!
//! Generates rays in world space from screen coordinates.
//! Future versions should also simulate depth of field.
//!
//! # Examples
//!
//! ```
//! use pathtrace::camera::Camera;
//! use pathtrace::core::{Vector3f, Vector2f, Vector2i};
//!
//! let cam = Camera::new(
//!     Vector3f::new(10.0), 
//!     Vector3f::new(0.0), 
//!     Vector3f::new_xyz(0.0, 1.0, 0.0), 
//!     90.0, Vector2i::new(10.0),
//!     );
//!
//! let (r, _) = cam.generate_ray(&Vector2f::new(5.0));
//! let dir = r.direction;
//!
//! assert!(
//!     dir.x == -0.44792563 &&
//!     dir.y == -0.659974 &&
//!     dir.z == -0.6031559
//!     );
//!
//! ```
use crate::Float;
use crate::core::{Vector3f, Vector2f, Vector2i, Ray};

/// A simple perspective camera
pub struct Camera {
    /// The camera origin in the screen
    origin: Vector3f,
    /// Vector from camera origin to the screen lower left corner
    screen_origin: Vector3f,
    /// Scaling vectors from screen_origin
    qx: Vector3f,
    qy: Vector3f,
}

impl Camera {
    /// Create a new camera look at a target
    ///
    /// The field of view specifies how wide the image should be.
    /// Currently must be between [0; 180[.
    pub fn new(origin: Vector3f, target: Vector3f, up: Vector3f, fov: Float, screensize: Vector2i) -> Camera {
        let screensize = Vector2f::from(screensize);
        // Calculate translation vectors
        let forward = (target - origin).norm();
        let right = up.cross(&origin).norm();
        let newup = forward.cross(&right).norm();

        // Calculate screen size from fov
        let aspect = screensize.y / screensize.x;
        let width = 2.0 * (fov / 2.0).to_radians().tan();
        let height = aspect * width;

        // Calculate screen scaling vectors
        let qx = right * (width / (screensize.x - 1.0));
        let qy = newup * (height / (screensize.y - 1.0));

        let screen_origin = forward - (right * (width/2.0)) - (newup * (height/2.0));

        Camera {
            origin,
            screen_origin,
            qx,
            qy,
        }
    }

    /// Generates a ray a screen space point
    ///
    /// The point coordinates should be between [0,0] (lower left corner) and [screensize.x,
    /// screensize.y] (upper right corner)
    ///
    /// Will return a ray and a weight
    ///
    /// The direction of the returned way is normalized
    pub fn generate_ray(&self, point: &Vector2f) -> (Ray, Float) {
        let mut dir = self.screen_origin + (self.qx * point.x) + (self.qy * point.y);
        dir.norm_in();

        (
            Ray { origin: self.origin, direction: dir },
            1.0
            )
    }
}
