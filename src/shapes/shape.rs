use crate::{Matrix, Point, Ray, Sphere, Vector};

use super::Plane;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
}

impl Shape {
    pub fn local_intersect(&self, ray: &Ray) -> Option<(f64, f64)> {
        match *self {
            Self::Sphere => Sphere::intersect(ray),
            Self::Plane => Plane::intersect(ray),
        }
    }

    pub fn local_normal_at(&self, point: &Point<f64, 4>) -> Vector<f64, 4> {
        match *self {
            Self::Sphere => Sphere::normal_at(point),
            Self::Plane => Plane::normal_at(point),
        }
    }
}
