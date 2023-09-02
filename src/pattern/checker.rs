use crate::{color::Color, ApproximateEq, Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checker {
    pub color_a: Color,
    pub color_b: Color,
}

impl Checker {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn checker_at(&self, point: &Point<f64, 4>) -> Color {
        let x = point.get(0).unwrap().floor();
        let y = point.get(1).unwrap().floor();
        let z = point.get(2).unwrap().floor();

        if ((x + y + z) as i64 % 2) == 0 {
            self.color_a.clone()
        } else {
            self.color_b.clone()
        }
    }
}
