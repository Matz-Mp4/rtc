//x + T::zero() == x
//Additive identity
//Inspiration: std::nums::zero
pub trait Zero: Sized {
    fn zero() -> Self;
}

macro_rules! default_impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            fn zero() -> $t {
                $v
            }
        }
    };
}
default_impl_zero! { usize, 0 }
default_impl_zero! { i8, 0 }
default_impl_zero! { i16, 0 }
default_impl_zero! { i32, 0 }
default_impl_zero! { i64, 0 }

default_impl_zero! { f32, 0.0f32 }
default_impl_zero! { f64, 0.0f64 }
