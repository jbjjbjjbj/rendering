use pathtrace::camera::Camera;
use pathtrace::core::{Vector3f, Vector2f};


fn main() {

    let cam = Camera::new(
        Vector3f::new(10.0), 
        Vector3f::new(0.0), 
        Vector3f::new_xyz(0.0, 1.0, 0.0), 
        90.0, Vector2f::new(10.0),
        );

    let (r, _) = cam.generate_ray(Vector2f::new(5.0));

    println!("r: {}, norm: {}", r.direction, r.direction.norm());
}
