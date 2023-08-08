use crate::ApproximateEq;
use crate::{Object, Ray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection {
    t: f64,
    object: Object,
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Self { t, object }
    }

    pub fn intersect(ray: &Ray, object: &Object) -> Option<(Intersection, Intersection)> {
        if let Some((t1, t2)) = object.shape.intersect(ray) {
            let inter1 = Intersection::new(t1, object.clone());
            let inter2 = Intersection::new(t2, object.clone());

            Some((inter1, inter2))
        } else {
            None
        }
    }
}

pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        Intersections {
            intersections: Vec::new(),
        }
    }
    pub fn from_ray(ray: &Ray, object: &Object) -> Self {
        let mut intersections = Vec::new();

        if let Some((inter1, inter2)) = Intersection::intersect(ray, object) {
            intersections.push(inter1);
            intersections.push(inter2);
        }
        Self { intersections }
    }

    pub fn from(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }

    pub fn add(&mut self, inter: Intersection) {
        self.intersections.push(inter);
    }

    pub fn get(&self, i: usize) -> Option<&Intersection> {
        self.intersections.get(i)
    }

    pub fn hit(&self) -> Option<&Intersection> {
        if let Some(mut value) = self.intersections.get(0) {
            for inter in &self.intersections {
                if value.t >= inter.t {
                    value = inter;
                }
            }

            if value.t > 0.0 {
                return Some(value);
            }
        }

        None
    }
}
