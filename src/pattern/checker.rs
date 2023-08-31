use crate::{color::Color, Point};

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
        let x = point.get(0).unwrap().floor() as i64;
        let y = point.get(1).unwrap().floor() as i64;
        let z = point.get(2).unwrap().floor() as i64;

        if (x + y + z) % 2 == 0 {
            self.color_a.clone()
        } else {
            self.color_b.clone()
        }
    }
}
