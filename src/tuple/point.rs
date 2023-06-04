use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

pub struct Point<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Point<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T: PartialEq, const N: usize> PartialEq for Point<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Clone + Copy, const N: usize> Add for Point<T, N>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = data.len();

        for i in 1..=len {
            data[i] = self.data[i] + rhs.data[i];
        }

        Point { data }
    }
}

impl<T: Clone + Copy, const N: usize> Sub for Point<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = data.len();

        for i in 1..=len {
            data[i] = self.data[i] - rhs.data[i];
        }

        Point { data }
    }
}

impl<T: Clone + Copy, const N: usize> Mul for Point<T, N>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = data.len();

        for i in 1..=len {
            data[i] = self.data[i] * rhs.data[i];
        }

        Point { data }
    }
}

impl<T: Clone + Copy, const N: usize> Div for Point<T, N>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = data.len();

        for i in 1..=len {
            data[i] = self.data[i] / rhs.data[i];
        }

        Point { data }
    }
}
