pub const BIG_EPSILON : f64= 1.0e-6;
pub const EPSILON : f64= 1.0e-3;


pub trait ApproximateEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
    fn approx_eq_low(&self, other: &Rhs) -> bool;
}

impl ApproximateEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < 1.0e-6
    }

    fn approx_eq_low(&self, other: &Self) -> bool {
        (self - other).abs() < 1.0e-3
    }
}

impl ApproximateEq for i32 {
    fn approx_eq(&self, other: &Self) -> bool {
        self == other
    }

    fn approx_eq_low(&self, other: &Self) -> bool {
        self == other
    }
}
