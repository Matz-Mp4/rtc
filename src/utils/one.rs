//x * T::one() == x
//Multiplicative identity
//Inspiration: std::nums::one
pub trait One: Sized {
    fn one() -> Self;
}

macro_rules! default_impl_one {
    ($t:ty, $v:expr) => {
        impl One for $t {
            fn one() -> $t {
                $v
            }
        }
    };
}
default_impl_one! { usize, 1 }
default_impl_one! { i8, 1 }
default_impl_one! { i16, 1 }
default_impl_one! { i32, 1 }
default_impl_one! { i64, 1 }

default_impl_one! { f32, 1.0f32 }
default_impl_one! { f64, 1.0f64 }
