pub trait Tuple<T> {
    fn zero() -> Self;
    fn get<'a>(i: usize) -> Option<&'a T>;
}
