use std::cmp::Ordering;

use crate::{Object, Point, Ray, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Self { t, object }
    }

    pub fn intersect(
        ray: &Ray,
        object: &'a Object,
    ) -> Option<(Intersection<'a>, Intersection<'a>)> {
        if let Some((t1, t2)) = object.shape.intersect(ray) {
            let inter1 = Intersection::new(t1, object);
            let inter2 = Intersection::new(t2, object);

            Some((inter1, inter2))
        } else {
            None
        }
    }

    pub fn prepare_computation(&self, ray: &Ray) -> Computations {
        let eye_vector = -ray.direction;
        let point1 = ray.position(self.t);
        let mut normal1 = self.object.normal_at(&point1);
        let inside1: bool;
        if normal1 * eye_vector < 0.0 {
            inside1 = true;
            normal1 = -normal1;
        } else {
            inside1 = false
        }

        let epsilon = 1.0e-6;

        let over_point1 = point1 + (normal1 * epsilon);
        Computations {
            t: self.t,
            object: self.object,
            point: point1,
            over_point: over_point1,
            eyev: eye_vector,
            normalv: normal1,
            inside: inside1,
        }
    }

    pub fn get_object(&self) -> &Object {
        self.object
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }
}

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Intersections {
            intersections: Vec::new(),
        }
    }
    pub fn from_ray(ray: &Ray, object: &'a Object) -> Self {
        let mut intersections = Vec::new();

        if let Some((inter1, inter2)) = Intersection::intersect(ray, object) {
            intersections.push(inter1);
            intersections.push(inter2);
        }
        Self { intersections }
    }

    pub fn from(intersections: Vec<Intersection<'a>>) -> Self {
        Self { intersections }
    }

    pub fn add(&mut self, inter: Intersection<'a>) {
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

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> std::cmp::Eq for Intersection<'a> {}

impl<'a> Ord for Intersection<'a> {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Point<f64, 4>,
    pub over_point: Point<f64, 4>,
    pub eyev: Vector<f64, 4>,
    pub normalv: Vector<f64, 4>,
    inside: bool,
}

impl<'a> Computations<'a> {
    pub fn new(
        t: f64,
        object: &'a Object,
        point: Point<f64, 4>,
        over_point: Point<f64, 4>,
        eyev: Vector<f64, 4>,
        normalv: Vector<f64, 4>,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}
