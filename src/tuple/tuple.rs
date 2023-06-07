pub trait Tuple<T: Copy> {
    fn zero(zero: T) -> Self;
    fn get<'a>(&'a self, i: usize) -> Option<&'a T>;
}


