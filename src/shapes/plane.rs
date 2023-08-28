use crate::{Point, Ray, Vector, BIG_EPSILON};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Plane {}
    }

    pub fn normal_at(object_point: &Point<f64, 4>) -> Vector<f64, 4> {
        Vector::new_vec3D(0.0, 1.0, 0.0)
    }

    pub fn intersect(ray: &Ray) -> Option<(f64, f64)> {
        let direction_y = ray.direction.get(1).unwrap();
        if direction_y.abs() < BIG_EPSILON {
            None
        } else {
            let origin_y = ray.origin.get(1).unwrap();
            let t = -origin_y / direction_y;
            Some((t, f64::INFINITY))
        }
    }
}
