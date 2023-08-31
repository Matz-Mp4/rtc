use crate::{color::Color, Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ring {
    pub color_a: Color,
    pub color_b: Color,
}

impl Ring {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn ring_at(&self, point: &Point<f64, 4>) -> Color {
        let x = point.get(0).unwrap();
        let z = point.get(2).unwrap();
        let temp = f64::sqrt(x * x + z * z) as i64;

        if temp % 2 == 0 {
            self.color_a.clone()
        } else {
            self.color_b.clone()
        }
    }
}
