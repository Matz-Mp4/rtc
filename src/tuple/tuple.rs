pub trait Tuple<T> {
    fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
        None
    }

    fn get_mut<'a>(&'a self, i: usize) -> Option<&'a mut T> {
        None
    }
    fn new() -> Self
    where
        Self: Sized;
}
