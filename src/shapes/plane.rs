use crate::{Point, Ray, Vector, EPSILON, ApproximateEq};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Plane {}
    }

    pub fn normal_at(object_point: &Point<f64, 4>) -> Vector<f64, 4> {
        Vector::new_vec3D(0.0, 1.0, 0.0)
    }

    pub fn intersect(ray: &Ray) -> Option<Vec<f64>> {
        let direction_y = ray.direction.get(1).unwrap();
        if direction_y.abs() < EPSILON || direction_y.approx_eq_low(&EPSILON) {
            None
        } else {
            let origin_y = ray.origin.get(1).unwrap();
            let t = -origin_y / direction_y;
            let mut result = Vec::new();
            result.push(t);
            Some(result)
        }
    }
}
