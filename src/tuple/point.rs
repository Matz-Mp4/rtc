use super::tuple::Tuple;
use super::vector::Vector;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Point<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
}

//---------------------------Tuple---------------------------
impl<T, const N: usize> Tuple<T> for Point<T, N>
where
    T: Copy,
{
    fn initialize(value: T) -> Self {
        let data = [value; N];
        Point { data }
    }

    fn get<'a>(&'a self, i: usize) -> Option<&'a T>
    where
        T: Copy,
    {
        if i < self.data.len() {
            let data = &(self.data[i]);
            return Some(data);
        } else {
            return None;
        }
    }
}

//---------------------------Point3---------------------------
impl<T> Point<T, 3> {
    pub fn new_point3(x: T, y: T, z: T) -> Self {
        Point { data: [x, y, z] }
    }
}

//---------------------------OverLoad---------------------------
impl<T: PartialEq, const N: usize> PartialEq for Point<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

//Scalar
impl<T: Clone + Copy, const N: usize> Neg for Point<T, N>
where
    T: Neg<Output = T>,
{
    type Output = Point<T, N>;
    fn neg(self) -> Self::Output {
        let mut data = self.data.clone();
        for i in 0..=self.data.len() - 1 {
            data[i] = -data[i];
        }

        Point { data }
    }
}
impl<T: Clone + Copy, const N: usize> Add for Point<T, N>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] + rhs.data[i];
        }

        Point { data }
    }
}

//Vector - Point = Point
impl<T, const N: usize> Sub<Vector<T, N>> for Point<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T, N>;
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] - *rhs.get(i).unwrap();
        }

        Point { data }
    }
}

impl<T: Clone + Copy, const N: usize> Sub for Point<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] - rhs.data[i];
        }
        let res = Vector::new(data);
        res
    }
}

//Scalar
impl<T: Clone + Copy, const N: usize> Mul<T> for Point<T, N>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] * rhs;
        }

        Point { data }
    }
}
//Scalar
impl<T: Clone + Copy, const N: usize> Div<T> for Point<T, N>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] / rhs;
        }

        Point { data }
    }
}
