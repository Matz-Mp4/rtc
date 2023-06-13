pub trait Sqrt<Rhs = Self> {
    type Output;
    fn sqrt(value: Rhs) -> Self::Output;
}

impl Sqrt<i32> for i32 {
    type Output = i32;
    fn sqrt(value: i32) -> Self::Output {
        let new_value = value as f32;
        new_value.sqrt() as i32
    }
}

impl Sqrt<i64> for i64 {
    type Output = i64;
    fn sqrt(value: i64) -> Self::Output {
        let new_value = value as f32;
        new_value.sqrt() as i64
    }
}

impl Sqrt<f64> for f64 {
    type Output = f64;
    fn sqrt(value: f64) -> Self::Output {
        value.sqrt()
    }
}

impl Sqrt<f32> for f32 {
    type Output = f32;
    fn sqrt(value: f32) -> Self::Output {
        value.sqrt()
    }
}

