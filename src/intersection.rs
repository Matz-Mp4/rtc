use crate::{Object, Ray};

pub struct Intersection {
    t: f64,
    object: Object,
    ray: Ray,
}

impl Intersection {
    pub fn new(t: f64, object: Object, ray: Ray) -> Self {
        Self { t, object, ray }
    }

    pub fn intersection(ray: &Ray, object: &Object) -> Option<(Intersection, Intersection)> {
        if let Some((t1, t2)) = object.shape.intersect(ray) {
            let inter1 = Intersection::new(t1, object.clone(), ray.clone());
            let inter2 = Intersection::new(t2, object.clone(), ray.clone());

            Some((inter1, inter2))
        } else {
            None
        }
    }
}

pub struct Intersections {
    intersections: Vec<Intersection>,
}
