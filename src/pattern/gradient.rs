use crate::{color::Color, Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gradient {
    pub color_a: Color,
    pub color_b: Color,
}

impl Gradient {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn gradient_at(&self, point: &Point<f64, 4>) -> Color {
        let distance = self.color_b - self.color_a;
        let x = point.get(0).unwrap();
        let fraction = x - x.floor();

        self.color_a + distance * fraction
    }
}
