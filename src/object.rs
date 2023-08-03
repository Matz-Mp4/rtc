use crate::Shape;

#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub shape: Shape,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self { shape }
    }
}
