pub trait Sqrt<T> {
    fn sqrt(value: T) -> T;
}

impl Sqrt<i32> for i32 {
    fn sqrt(value: i32) -> i32 {
        let new_value = value as f32;
        new_value.sqrt() as i32
    }
}

impl Sqrt<i64> for i64 {
    fn sqrt(value: i64) -> i64 {
        let new_value = value as f32;
        new_value.sqrt() as i64
    }
}

impl Sqrt<f64> for f64 {
    fn sqrt(value: f64) -> f64 {
        value.sqrt()
    }
}

impl Sqrt<f32> for f32 {
    fn sqrt(value: f32) -> f32 {
        value.sqrt()
    }
}
