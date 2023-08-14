use crate::transformation::scaling;
use crate::Object;
use crate::{color::Color, Light, Point};

pub struct World {
    light: Light,
    objects: Vec<Object>,
}

impl World {
    pub fn new(light: Light, objects: Vec<Object>) -> Self {
        Self { light, objects }
    }

    pub fn default() -> World {
        let light_positon = Point::new_point3D(-10.0, 10.0, -10.0);
        let light_color = Color::white();
        let light = Light::new(light_color, light_positon);

        let mut ob1 = Object::new_sphere();
        ob1.set_specular(0.2);
        ob1.set_difuse(0.7);
        ob1.set_color(Color::new(0.8,1.0,0.6));

        let mut ob2 = Object::new_sphere();
        ob2.set_transformation(scaling(0.5, 0.5, 0.5));
        let objects = Vec::from([ob1, ob2]);

        World { light, objects }
    }
}
