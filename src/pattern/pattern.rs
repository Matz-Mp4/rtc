use crate::{color::Color, Matrix, Point, Striped};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pattern {
    pub p_type: PatternType,
    pub transformation: Matrix<f64, 4, 4>,
    pub inverse: Matrix<f64, 4, 4>,
}

impl Pattern {
    pub fn new(
        p_type: PatternType,
        transformation: Matrix<f64, 4, 4>,
        inverse: Matrix<f64, 4, 4>,
    ) -> Self {
        Self {
            p_type,
            transformation,
            inverse,
        }
    }

    pub fn with_type(p_type: PatternType) -> Pattern {
        let iden = Matrix::iden();
        Pattern::new(p_type, iden, iden)
    }

    pub fn set_type(&mut self, p_type: PatternType) {
        self.p_type = p_type
    }
    pub fn set_transformation(&mut self, transformation: Matrix<f64, 4, 4>) {
        self.transformation = transformation;
        self.inverse = transformation.inverse();
    }

    pub fn pattern_at(&self, point: &Point<f64, 4>) -> Option<Color> {
        self.p_type.pattern_at(point)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PatternType {
    Striped(Striped),
    None,
}

impl PatternType {
    pub fn striped_pattern(color_a: Color, color_b: Color) -> PatternType {
        PatternType::Striped(Striped::new(color_a, color_b))
    }

    pub fn pattern_at(&self, point: &Point<f64, 4>) -> Option<Color> {
        match self {
            PatternType::Striped(p) => Some(p.stripe_at(point)),
            PatternType::None => None,
        }
    }
}
