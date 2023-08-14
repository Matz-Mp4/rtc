use crate::{color::Color, Material, Matrix, Point, Ray, Shape, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub material: Material,
    transformation: Matrix<f64, 4, 4>,
    inverse_transformation: Matrix<f64, 4, 4>,
    inverse_transpose: Matrix<f64, 4, 4>,
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

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn set_color(&mut self, color: Color) {
        self.material.set_color(color);
    }

    pub fn set_difuse(&mut self, diffuse: f64) {
        self.material.set_diffuse(diffuse);
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.material.set_specular(specular);
    }

    pub fn set_transformation(&mut self, transformation: Matrix<f64, 4, 4>) {
        let inverse = transformation.inverse();
        let transpose = inverse.trans();
        self.transformation = transformation;
        self.inverse_transformation = inverse;
        self.inverse_transpose = transpose;
    }

    pub fn intersects(&self, ray: &Ray) -> Option<(f64, f64)> {
        let transformed_ray = ray.transform(&self.inverse_transformation);
        self.shape.intersect(&transformed_ray)
    }

    pub fn normal_at(&self, point: &Point<f64, 4>) -> Vector<f64, 4> {
        let object_point = self.inverse_transformation * (*point);
        let object_normal = object_point - Point::new_point3D(0.0, 0.0, 0.0);
        let mut world_normal = self.inverse_transpose * object_normal;
        let w = world_normal.get_mut(3).unwrap();
        *w = 0.0;
        world_normal.normalize()

        /* self.shape.normal_at(point) */
    }
}
