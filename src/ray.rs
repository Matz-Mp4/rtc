use crate::{Matrix, Point, Tuple, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point<f64, 4>,
    pub direction: Vector<f64, 4>,
}

impl Ray {
    pub fn new(ori: impl Tuple<f64, 4>, dir: impl Tuple<f64, 4>) -> Self {
        let p_x = *ori.get(0).unwrap();
        let p_y = *ori.get(1).unwrap();
        let p_z = *ori.get(2).unwrap();

        let v_x = *dir.get(0).unwrap();
        let v_y = *dir.get(1).unwrap();
        let v_z = *dir.get(2).unwrap();

        Self {
            origin: Point::new_point3D(p_x, p_y, p_z),
            direction: Vector::new_vec3D(v_x, v_y, v_z),
        }
    }

    pub fn transform(&self, transformation: &Matrix<f64, 4, 4>) -> Ray {
        Ray {
            origin: *transformation * self.origin,
            direction: *transformation * self.direction,
        }
    }

    pub fn position(&self, t: f64) -> Point<f64, 4> {
        self.origin + self.direction * t
    }
}
