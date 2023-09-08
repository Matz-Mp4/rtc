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

    pub fn get_mut_object(&mut self, i: usize) -> Option<&mut Object> {
        self.objects.get_mut(i)
    }

    pub fn default_test() -> World {
        let light_positon = Point::new_point3D(-10.0, 10.0, -10.0);
        let light_color = Color::white();
        let light = Light::new(light_color, light_positon);

        let mut ob1 = Object::new_sphere();
        ob1.material.color = Color::new(0.8, 1.0, 0.6);
        ob1.material.diffuse = 0.7;
        ob1.material.specular = 0.2;

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

    pub fn shade_hit(&self, comps: &Computations, remaing: u8) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);
        let surface = comps.object.material.lightning(
            comps.object,
            &self.light,
            &comps.over_point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(&comps, remaing);
        let refracted = self.refracted_color(&comps, remaing);

        if comps.object.material.reflective > 0.0 && comps.object.material.transparency > 0.0 {
            let reflectance = comps.schlick();
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        } else {
            return surface + reflected + refracted;
        }
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

    pub fn reflected_color(&self, comps: &Computations, remaing: u8) -> Color {
        if comps.object.material.reflective.approx_eq_low(&0.0) || remaing == 0 {
            Color::black()
        } else {
            let relfect_ray = Ray::new(comps.over_point.clone(), comps.reflectv.clone());
            let color = self.color_at(&relfect_ray, remaing - 1);

            color * comps.object.material.reflective
        }
    }

    pub fn refracted_color(&self, comps: &Computations, remaing: u8) -> Color {
        if comps.object.material.transparency.approx_eq(&0.0) || remaing == 0 {
            Color::black()
        } else {
            //this is inverted from the definition of Snell's Law.
            let n_ratio = comps.n1 / comps.n2;
            // cos(theta_i) is the same as the dot product of the two vectors
            let cos_i = comps.eyev * comps.normalv;
            // sin(theta_t)^2 via trigonometric identity
            let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

            if sin2_t > 1.0 {
                return Color::black();
            }
            //cos(theta_t) via trigonometric identity
            let cos_t = f64::sqrt(1.0 - sin2_t);
            //Compute the direction of the refracted ray
            let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
            //Create the refracted ray
            let refract_ray = Ray::new(comps.under_point, direction);

            let color =
                self.color_at(&refract_ray, remaing - 1) * comps.object.material.transparency;

            color
        }
    }

    pub fn color_at(&self, ray: &Ray, remaing: u8) -> Color {
        let mut color = Color::black();
        let inters = self.intersect_world(ray);

        if let Some(hit) = inters.hit_index() {
            let comps = Computations::prepare_computation(&inters, hit, ray);
            color = self.shade_hit(&comps, remaing);
        }
        color
    }
}
