use crate::intersection::Computations;
use crate::transformation::scaling;
use crate::ApproximateEq;
use crate::{color::Color, Light, Point};
use crate::{Intersection, Intersections, Object, Ray};

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub light: Light,
    pub objects: Vec<Object>,
    pub reflection_limit: u8,
}

impl World {
    pub fn new(light: Light, objects: Vec<Object>, reflection_limit: u8) -> Self {
        Self {
            light,
            objects,
            reflection_limit,
        }
    }

    pub fn push_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_object(&self, i: usize) -> Option<&Object> {
        self.objects.get(i)
    }

    pub fn default() -> World {
        let light_positon = Point::new_point3D(-10.0, 10.0, -10.0);
        let light_color = Color::white();
        let light = Light::new(light_color, light_positon);

        let mut ob1 = Object::new_sphere();
        ob1.set_color(Color::new(0.8, 1.0, 0.6));
        ob1.set_difuse(0.7);
        ob1.set_specular(0.2);

        let mut ob2 = Object::new_sphere();
        ob2.set_transformation(scaling(0.5, 0.5, 0.5));
        let objects = Vec::from([ob1, ob2]);

        World {
            light,
            objects,
            reflection_limit: 4,
        }
    }

    pub fn intersect_world(&self, ray: &Ray) -> Intersections {
        let mut inters = Intersections::new();

        for object in &self.objects {
            if let Some(result) = object.intersects(ray) {
                for t in result {
                    inters.add(Intersection::new(t, &object));
                }
            }
        }
        inters.sort();

        inters
    }

    pub fn shade_hit(&self, comps: &Computations) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);
        let surface = comps.object.material.lightning(
            comps.object,
            &self.light,
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(&comps);

        surface + reflected
    }

    pub fn is_shadowed(&self, point: &Point<f64, 4>) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let mut shadowed = false;

        let r = Ray::new(*point, direction);
        let mut inters = self.intersect_world(&r);

        if let Some(hit) = inters.hit() {
            if hit.get_t() < distance {
                shadowed = true;
            }
        }

        shadowed
    }

    pub fn reflected_color(&self, comps: &Computations) -> Color {
        if comps.object.material.reflective.approx_eq_low(&0.0) || self.reflection_limit == 0 {
            Color::black()
        } else {
            let relfect_ray = Ray::new(comps.over_point.clone(), comps.reflectv.clone());
            let color = self.color_at(&relfect_ray);

            color * comps.object.material.reflective
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let mut color = Color::black();
        let mut inters = self.intersect_world(ray);

        if let Some(hit) = inters.hit() {
            let comps = hit.prepare_computation(&Intersections::new(), 0, ray);
            color = self.shade_hit(&comps);
        }
        color
    }
}
