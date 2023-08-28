use crate::{Matrix, Point, Ray, Vector};

// We assume a sphere is always at Position{0, 0 , 0}, thus the absence of coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }
    pub fn intersect(ray: &Ray) -> Option<(f64, f64)> {
        let sphere_to_ray = ray.origin - Point::new_point3D(0.0, 0.0, 0.0);
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;

        let discriminat = b.powi(2) - 4.0 * a * c;

        if discriminat < 0.0 {
            None
        } else {
            let t1 = (-b - discriminat.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminat.sqrt()) / (2.0 * a);
            Some((t1, t2))
        }
    }

    pub fn normal_at(object_point: &Point<f64, 4>) -> Vector<f64, 4> {
        let object_normal = *object_point - Point::new_point3D(0.0, 0.0, 0.0);
        object_normal
    }
}
