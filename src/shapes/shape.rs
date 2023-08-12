use crate::{Point, Ray, Sphere, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Sphere,
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Option<(f64, f64)> {
        match *self {
            Self::Sphere => Sphere::intersect(ray),
        }
    }

    pub fn normal_at(&self, point: &Point<f64, 4>) -> Vector<f64, 4> {
        match *self {
            Self::Sphere => Sphere::normal_at(point),
        }
    }
}
