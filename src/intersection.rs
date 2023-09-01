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

    pub fn intersect(ray: &Ray, object: &'a Object) -> Option<Intersections<'a>> {
        if let Some(inters) = object.shape.local_intersect(ray) {
            let mut output = Intersections::new();

            for t in inters {
                output.add(Intersection::new(t, object));
            }

            Some(output)
        } else {
            None
        }
    }

    pub fn prepare_computation(
        &self,
        intersections: &Intersections<'a>,
        intersection_index: usize,
        ray: &Ray,
    ) -> Computations {
        let mut containers: Vec<&Object> = Vec::new();
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        for inter in intersections.into_iter() {
            if let Some(k) = intersections.hit() {
                if inter == *k {
                    if containers.is_empty() {
                        n1 = 1.0;
                    } else {
                        let last = containers.last().unwrap();
                        n1 = last.material.refractive_index;
                    }
                }
            }

            match containers
                .iter()
                .position(|&object| std::ptr::eq(object, inter.object))
            {
                Some(pos) => {
                    let _ = containers.remove(pos);
                }
                None => containers.push(inter.object),
            }

            if let Some(k) = intersections.hit() {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    let last = containers.last().unwrap();
                    n2 = last.material.refractive_index;
                }
            }
        }
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
        let reflectv1 = ray.direction.reflect(&normal1);
        let epsilon = 1.0e-6;
        let over_point1 = point1 + (normal1 * epsilon);

        Computations {
            t: self.t,
            n1,
            n2,
            object: self.object,
            point: point1,
            over_point: over_point1,
            eyev: eye_vector,
            normalv: normal1,
            reflectv: reflectv1,
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
    /*
    pub fn from_ray(ray: &Ray, object: &'a Object) -> Self {
        let mut intersections = Vec::new();

        if let Some((inter1, inter2)) = Intersection::intersect(ray, object) {
            intersections.push(inter1);
            intersections.push(inter2);
        }
        Self { intersections }
    }*/

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

    pub fn into_iter(&self) -> std::vec::IntoIter<Intersection> {
        self.intersections.clone().into_iter()
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
    pub n1: f64,
    pub n2: f64,
    pub object: &'a Object,
    pub point: Point<f64, 4>,
    pub over_point: Point<f64, 4>,
    pub eyev: Vector<f64, 4>,
    pub normalv: Vector<f64, 4>,
    pub reflectv: Vector<f64, 4>,
    inside: bool,
}

impl<'a> Computations<'a> {
    pub fn new(
        t: f64,
        n1: f64,
        n2: f64,
        object: &'a Object,
        point: Point<f64, 4>,
        over_point: Point<f64, 4>,
        eyev: Vector<f64, 4>,
        normalv: Vector<f64, 4>,
        reflectv: Vector<f64, 4>,
        inside: bool,
    ) -> Self {
        Self {
            t,
            n1,
            n2,
            object,
            point,
            over_point,
            eyev,
            normalv,
            reflectv,
            inside,
        }
    }
}
