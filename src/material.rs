use crate::{color::Color, Light, Object, Pattern, PatternType, Point, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub ambient: f64,
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub pattern: Pattern,
}

impl Material {
    pub fn new(
        ambient: f64,
        color: Color,
        diffuse: f64,
        specular: f64,
        shininess: f64,

        reflective: f64,
        pattern: Pattern,
    ) -> Self {
        Self {
            ambient,
            color,
            diffuse,
            specular,
            shininess,
            reflective,
            pattern,
        }
    }

    pub fn default() -> Material {
        Material::new(
            0.1,
            Color::white(),
            0.9,
            0.9,
            200.0,
            0.0,
            Pattern::with_type(PatternType::None),
        )
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse;
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular;
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = pattern;
    }

    pub fn lightning(
        &self,
        object: &Object,
        light: &Light,
        point: &Point<f64, 4>,
        eyev: &Vector<f64, 4>,
        normalv: &Vector<f64, 4>,
        shadowed: bool,
    ) -> Color {
        let diffuse: Color;
        let specular: Color;

        let color = {
            if self.pattern.p_type != PatternType::None {
                object.pattern_at_object(point).unwrap()
            } else {
                self.color
            }
        };

        let effective_color = color * light.intensity;
        let lightv = Vector::normalize(light.position - *point);
        let ambient = effective_color * self.ambient;

        if shadowed {
            return ambient;
        }

        //light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv * *normalv;

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            //reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = Vector::reflect(&(-lightv), &normalv);
            let reflect_dot_eye = reflectv * *eyev;

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}
