use crate::{color::Color, transformation::translation, Motion, Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub intensity: Color,
    pub position: Point<f64, 4>,
}

impl Light {
    pub fn new(intensity: Color, position: Point<f64, 4>) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

impl Motion for Light {
    fn move_front(&mut self, value: f64) -> Self {
        self.position = translation(0.0, 0.0, value) * self.position;
        *self
    }

    fn move_back(&mut self, value: f64) -> Self {
        self.position = translation(0.0, 0.0, -value) * self.position;
        *self
    }

    fn move_left(&mut self, value: f64) -> Self {
        self.position = translation(-value, 0.0, 0.0) * self.position;
        *self
    }

    fn move_right(&mut self, value: f64) -> Self {
        self.position = translation(value, 0.0, 0.0) * self.position;
        *self
    }

    fn move_up(&mut self, value: f64) -> Self {
        self.position = translation(0.0, value, 0.0) * self.position;
        *self
    }

    fn move_down(&mut self, value: f64) -> Self {
        self.position = translation(0.0, -value, 0.0) * self.position;
        *self
    }

    fn look_left(&mut self, value: f64) -> Self {
        todo!()
    }

    fn look_right(&mut self, value: f64) -> Self {
        todo!()
    }

    fn look_up(&mut self, value: f64) -> Self {
        todo!()
    }

    fn look_down(&mut self, value: f64) -> Self {
        todo!()
    }
}
