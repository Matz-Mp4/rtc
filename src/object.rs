use crate::transformation::*;
use crate::{color::Color, Material, Matrix, Motion, Pattern, Point, Ray, Shape, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub material: Material,
    transformation: Matrix<f64, 4, 4>,
    inverse_transformation: Matrix<f64, 4, 4>,
    inverse_transpose: Matrix<f64, 4, 4>,
}

impl Motion for Object {
    fn move_front(&mut self, value: f64) -> Self {
        let move_front = translation(0.0, 0.0, value) * self.transformation;
        self.set_transformation(move_front);
        *self
    }

    fn move_back(&mut self, value: f64) -> Self {
        let move_front = translation(0.0, 0.0, -value) * self.transformation;
        self.set_transformation(move_front);
        *self
    }

    fn move_left(&mut self, value: f64) -> Self {
        let move_front = translation(-value, 0.0, 0.0) * self.transformation;
        self.set_transformation(move_front);
        *self
    }

    fn move_right(&mut self, value: f64) -> Self {
        let move_front = translation(value, 0.0, 0.0) * self.transformation;
        self.set_transformation(move_front);
        *self
    }

    fn move_up(&mut self, value: f64) -> Self {
        let move_front = translation(0.0, value, 0.0) * self.transformation;
        self.set_transformation(move_front);
        *self
    }

    fn move_down(&mut self, value: f64) -> Self {
        let move_front = translation(0.0, -value, 0.0) * self.transformation;
        self.set_transformation(move_front);
        *self
    }
    fn look_left(&mut self, degree: f64) -> Self {
        let look = rotation_y(degree.to_radians()) * self.transformation;
        self.set_transformation(look);
        *self
    }

    fn look_right(&mut self, degree: f64) -> Self {
        let look = rotation_y(-degree.to_radians()) * self.transformation;
        self.set_transformation(look);
        *self
    }

    fn look_up(&mut self, degree: f64) -> Self {
        let look = rotation_x(-degree.to_radians()) * self.transformation;
        self.set_transformation(look);
        *self
    }

    fn look_down(&mut self, degree: f64) -> Self {
        let look = rotation_x(degree.to_radians()) * self.transformation;
        self.set_transformation(look);
        *self
    }
}

impl Object {
    pub fn new(shape: Shape, transformation: Matrix<f64, 4, 4>) -> Self {
        let inverse = transformation.inverse();
        let transpose = inverse.trans();
        Self {
            shape,
            material: Material::default(),
            transformation,
            inverse_transformation: inverse,
            inverse_transpose: transpose,
        }
    }
    pub fn new_plane() -> Self {
        let iden = Matrix::iden();
        Self {
            shape: Shape::Plane,
            material: Material::default(),
            transformation: iden,
            inverse_transformation: iden,
            inverse_transpose: iden,
        }
    }

    pub fn new_glass_sphere() -> Self {
        let iden = Matrix::iden();
        let mut material = Material::default();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        Self {
            shape: Shape::Sphere,
            material,
            transformation: iden,
            inverse_transformation: iden,
            inverse_transpose: iden,
        }
    }

    pub fn new_sphere() -> Self {
        let iden = Matrix::iden();
        Self {
            shape: Shape::Sphere,
            material: Material::default(),
            transformation: iden,
            inverse_transformation: iden,
            inverse_transpose: iden,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.material.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.material.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.material.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.material.specular = specular;
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.material.pattern = pattern;
        self
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn with_transformation(mut self, transformation: Matrix<f64, 4, 4>) -> Self {
        let inverse = transformation.inverse();
        let transpose = inverse.trans();
        self.transformation = transformation;
        /* self.material.pattern.transformation= transformation; */
        /* self.material.pattern.inverse= inverse; */
        self.inverse_transformation = inverse;
        self.inverse_transpose = transpose;

        self
    }

    pub fn set_transformation(&mut self, transformation: Matrix<f64, 4, 4>) {
        let inverse = transformation.inverse();
        let transpose = inverse.trans();
        self.transformation = transformation;
        /* self.material.pattern.transformation= transformation; */
        /* self.material.pattern.inverse= inverse; */
        self.inverse_transformation = inverse;
        self.inverse_transpose = transpose;
    }

    pub fn intersects(&self, ray: &Ray) -> Option<Vec<f64>> {
        let local_ray = ray.transform(&self.inverse_transformation);
        self.shape.local_intersect(&local_ray)
    }

    pub fn normal_at(&self, point: &Point<f64, 4>) -> Vector<f64, 4> {
        let local_point = self.inverse_transformation * *point;
        let local_normal = self.shape.local_normal_at(&local_point);
        let mut world_normal = self.inverse_transpose * local_normal;
        let w = world_normal.get_mut(3).unwrap();
        *w = 0.0;
        world_normal.normalize()
    }

    pub fn pattern_at_object(&self, world_point: &Point<f64, 4>) -> Option<Color> {
        let object_point = self.inverse_transformation * *world_point;
        let pattern_point = self.material.pattern.inverse * object_point;

        self.material.pattern.pattern_at(&pattern_point)
    }
}
