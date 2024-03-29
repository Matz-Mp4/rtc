use crate::ApproximateEq;

use super::super::One;
use super::super::Zero;
use super::tuple::Tuple;
use super::vector::Vector;
use std::cmp::PartialEq;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::{Add, Div, Mul, Neg, Sub};


#[derive(Clone, Copy, Debug)]
pub struct Point<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Point<T, N> {
    pub fn from(data: [T; N]) -> Self {
        Self { data }
    }

    pub fn new() -> Self
    where
        T: Copy + Sized + Zero,
    {
        let data: [T; N] = [Zero::zero(); N];

        Point { data }
    }

    pub fn get<'a>(&'a self, i: usize) -> Option<&'a T>
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

    pub fn get_mut<'a>(&'a mut self, i: usize) -> Option<&'a mut T>
    where
        T: Copy,
    {
        if i < self.data.len() {
            let data = &mut (self.data[i]);
            return Some(data);
        } else {
            return None;
        }
    }
}

//---------------------------Tuple---------------------------
impl<T, const N: usize> Tuple<T, N> for Point<T, N>
where
    T: Copy + Zero,
{
    fn new() -> Self {
        Point::new()
    }

    fn get<'a>(&'a self, i: usize) -> Option<&'a T>
    where
        T: Copy,
    {
        self.get(i)
    }

    fn get_mut<'a>(&'a mut self, i: usize) -> Option<&'a mut T>
    where
        T: Copy,
    {
        self.get_mut(i)
    }

    fn from(data: [T; N]) -> Self
    where
        Self: Sized,
    {
        Self { data }
    }
}

//---------------------------Point3---------------------------
impl<T> Point<T, 3> {
    pub fn new_point3(x: T, y: T, z: T) -> Self {
        Point { data: [x, y, z] }
    }
}

impl<T> Point<T, 4>
where
    T: Copy + One,
{
    pub fn new_point3D(x: T, y: T, z: T) -> Self {
        Point {
            data: [x, y, z, One::one()],
        }
    }
}

//---------------------------OverLoad---------------------------
impl<T: ApproximateEq + PartialEq, const N: usize> PartialEq for Point<T, N> {
    fn eq(&self, other: &Self) -> bool {
        let mut res = false;
        let len = self.data.len() - 1;
        for i in 0..=len {
            res = self.data[i].approx_eq_low(&other.data[i]);
            if !res {
                break;
            }
        }
        res
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
    Vector<T, N>: Tuple<T, N>,
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

//Vector + Point = Point
impl<T, const N: usize> Add<Vector<T, N>> for Point<T, N>
where
    T: Add<Output = T> + Copy,
    Vector<T, N>: Tuple<T, N>,
{
    type Output = Point<T, N>;
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] + *rhs.get(i).unwrap();
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
        let res = Vector::from(data);
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

impl<T, const N: usize> Index<usize> for Point<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Point<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}
