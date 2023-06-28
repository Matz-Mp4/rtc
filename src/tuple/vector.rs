use core::ops::Deref;
use std::cmp::PartialEq;
use std::ops::{Add, BitOr, Div, Mul, Neg, Sub};

use super::super::ApproximateEq;
use super::super::Zero;
use super::point::Point;
use super::tuple::Tuple;
use super::utils::Sqrt;

#[derive(Clone, Copy, Debug)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new() -> Self
    where
        T: Copy + Zero,
    {
        let data: [T; N] = [Zero::zero(); N];

        Self { data }
    }

    pub fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T: Sqrt<Output = T>, const N: usize> Vector<T, N> {
    pub fn magnitude(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        let mut res = self.data[0] * self.data[0];
        for i in 1..=self.data.len() - 1 {
            res = self.data[i] * self.data[i] + res;
        }
        <T as Sqrt>::sqrt(res)
    }

    pub fn normalize(self) -> Self
    where
        T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
    {
        let mut data = self.data.clone();
        for i in 0..=self.data.len() - 1 {
            data[i] = self.data[i] / self.magnitude();
        }
        Vector { data }
    }
}

//---------------------------Tuple---------------------------
impl<T, const N: usize> Tuple<T, N> for Vector<T, N>
where
    T: Zero + Copy,
{
    fn new() -> Self {
        Vector::new()
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

    fn from(data: [T; N]) -> Self
    where
        Self: Sized,
    {
        Self { data }
    }
}

//---------------------------Vector3---------------------------
impl<T> Vector<T, 3> {
    pub fn new_vec3(x: T, y: T, z: T) -> Self {
        Vector { data: [x, y, z] }
    }
    pub fn cross_produtc(&self, other: Self) -> Vector<T, 3>
    where
        T: Mul<Output = T> + Copy + Sub<Output = T>,
    {
        let x = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        let y = self.data[2] * other.data[0] - self.data[0] * other.data[2];
        let z = self.data[0] * other.data[1] - self.data[1] * other.data[0];

        Vector { data: [x, y, z] }
    }
}

//---------------------------OverLoad---------------------------
impl<T: ApproximateEq + PartialEq, const N: usize> PartialEq for Vector<T, N> {
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

impl<T: Clone + Copy, const N: usize> Add for Vector<T, N>
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

        Vector { data }
    }
}
//Vector - Vector = Vector
impl<T: Clone + Copy, const N: usize> Sub<Vector<T, N>> for Vector<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] - rhs.data[i];
        }

        Vector { data }
    }
}

//Vector - Point = Point
impl<T: Clone + Copy, const N: usize> Sub<Point<T, N>> for Vector<T, N>
where
    T: Sub<Output = T> + Zero,
{
    type Output = Point<T, N>;
    fn sub(self, rhs: Point<T, N>) -> Point<T, N> {
        let mut data = self.data.clone();
        let len = self.data.len() - 1;

        for i in 0..=len {
            data[i] = self.data[i] - *rhs.get(i).unwrap();
        }

        let res = Point::from(data);

        res
    }
}

//Scalar
impl<T: Clone + Copy, const N: usize> Neg for Vector<T, N>
where
    T: Neg<Output = T>,
{
    type Output = Vector<T, N>;
    fn neg(self) -> Self::Output {
        let mut data = self.data.clone();
        for i in 0..=self.data.len() - 1 {
            data[i] = -data[i];
        }

        Vector { data }
    }
}

//Scalar
impl<T: Clone + Copy, const N: usize> Mul<T> for Vector<T, N>
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

        Vector { data }
    }
}
//Scalar
impl<T: Clone + Copy, const N: usize> Div<T> for Vector<T, N>
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

        Vector { data }
    }
}

//Dot Product
impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Mul<Output = T> + Add<Output = T>,
    T: Clone + Copy,
{
    type Output = T;
    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut res = self.data[0] * rhs.data[0];
        let len = self.data.len() - 1;

        for i in 1..=len {
            res = self.data[i] * rhs.data[i] + res;
        }

        res
    }
}

//Cross Product
impl<T> BitOr for Vector<T, 3>
where
    T: Mul<Output = T> + Copy + Sub<Output = T>,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let x = self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1];
        let y = self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2];
        let z = self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0];

        Vector { data: [x, y, z] }
    }
}

impl<T: Copy + Zero + Sized, const N: usize> Deref for Vector<&T, N> {
    type Target = Vector<T, N>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
