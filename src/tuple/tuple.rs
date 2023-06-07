
pub trait Tuple<T> where T: Copy {
    const ZERO : T;

    fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
        None
    }
    fn zero() -> Self;
}
