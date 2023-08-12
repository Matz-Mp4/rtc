use crate::{Point, color::Color};

pub struct Light {
    pub intensity: Color,
    pub position: Point<f64, 4>
}

impl Light {
    pub fn new(intensity: Color, position: Point<f64, 4>)  -> Self{
        Self {
            intensity,
            position
        }
    }

} 
