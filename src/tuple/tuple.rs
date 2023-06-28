pub trait Tuple<T: Sized, const N: usize> {
    fn get<'a>(&'a self, _i: usize) -> Option<&'a T> {
        None
    }

    fn get_mut<'a>(&'a self, _i: usize) -> Option<&'a mut T> {
        None
    }
    fn new() -> Self
    where
        Self: Sized;

    fn from(data: [T; N]) -> Self
    where
        Self: Sized;
}
