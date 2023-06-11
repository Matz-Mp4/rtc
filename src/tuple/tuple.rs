pub trait Tuple<T> {
    fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
        None
    }
    fn initialize(value: T) -> Self;
}
