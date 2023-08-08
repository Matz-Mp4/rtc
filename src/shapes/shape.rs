use crate::{Ray, Sphere};

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
}
