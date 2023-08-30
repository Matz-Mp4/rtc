use crate::{color::Color, Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Striped {
    color_a: Color,
    color_b: Color,
}

impl Striped {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn stripe_at(&self, point: &Point<f64, 4>) -> Color {
        let x = point.get(0).unwrap();
        let temp = x.floor() as i32;
        if temp % 2 == 0 {
            self.color_a.clone()
        } else {
            self.color_b.clone()
        }
    }

}
