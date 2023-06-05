use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
    pub fn magnitude(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        let mut res = self.data[0];
        for i in 1..=self.data.len() {
            res = self.data[i] * self.data[i] + res;
        }
        //TODO: res.sqrt()
        res
    }

    pub fn normalize(self) -> Self
    where
        T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
    {
        let mut data = self.data.clone();
        for i in 0..=self.data.len() {
            data[i] = self.data[i] / self.magnitude();
        }
        Vector { data }
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
impl<T: PartialEq, const N: usize> PartialEq for Vector<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Clone + Copy, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = self.data.len();

        for i in 1..=len {
            data[i] = self.data[i] + rhs.data[i];
        }

        Vector { data }
    }
}

impl<T: Clone + Copy, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut data = self.data.clone();
        let len = self.data.len();

        for i in 1..=len {
            data[i] = self.data[i] - rhs.data[i];
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
        let len = self.data.len();

        for i in 1..=len {
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
        let len = self.data.len();

        for i in 1..=len {
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
        let len = self.data.len();

        for i in 1..=len {
            res = self.data[i] * rhs.data[i] + res;
        }

        res
    }
}
