use std::cmp::Ordering;

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

    pub fn sort(&mut self) {
        self.intersections.sort_unstable();
    }

    pub fn hit(&mut self) -> Option<&Intersection> {
        self.intersections.sort_unstable();
        self.intersections.iter().find(|i| i.t >= 0.0)
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Eq for Intersection {}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else if self.t < other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
