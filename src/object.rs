use crate::{Matrix, Ray, Shape};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    transformation: Matrix<f64, 4, 4>,
    inverse_transformation: Matrix<f64, 4, 4>,
}

impl Object {
    pub fn new(shape: Shape, transformation: Matrix<f64, 4, 4>) -> Self {
        Self {
            shape,
            transformation,
            inverse_transformation: transformation.inverse::<3>(),
        }
    }

    pub fn new_sphere() -> Self {
        Self {
            shape: Shape::Sphere,
            transformation: Matrix::iden(),
            inverse_transformation: Matrix::iden(),
        }
    }

    pub fn set_transformation(&mut self, transformation: Matrix<f64, 4, 4>) {
        self.transformation = transformation;
        self.inverse_transformation = transformation.inverse::<3>();

    }

    pub fn intersects(&self, ray: &Ray) -> Option<(f64, f64)> {
        let transformed_ray = ray.transform(&self.inverse_transformation);
        self.shape.intersect(&transformed_ray)
    }
}
