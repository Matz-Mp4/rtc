use crate::intersection::Computations;
use crate::transformation::scaling;
use crate::{color::Color, Light, Point};
use crate::{Intersection, Intersections, Object, Ray};

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub light: Light,
    pub objects: Vec<Object>,
}

impl World {
    pub fn new(light: Light, objects: Vec<Object>) -> Self {
        Self { light, objects }
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

        World { light, objects }
    }

    pub fn intersect_world(&self, ray: &Ray) -> Intersections {
        let mut inters = Intersections::new();

        for object in &self.objects {
            if let Some((t1, t2)) = object.intersects(ray) {
                inters.add(Intersection::new(t1, &object));
                inters.add(Intersection::new(t2, &object));
            }
        }
        inters.sort();

        inters
    }


    pub fn shade_hit(&self, comps: &Computations) -> Color {
        comps
            .object
            .material
            .lightning(&self.light, &comps.point, &comps.eyev, &comps.normalv)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let mut color = Color::black();
        let mut inters = self.intersect_world(ray);

        let empty = inters.is_empty();
        if let Some(hit) = inters.hit(){
            let comps = hit.prepare_computation(ray);
            color = self.shade_hit(&comps);
        }
        color
    }
}
