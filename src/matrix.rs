use crate::ApproximateEq;
use std::ops::{Add, Div, Mul, Sub};

type Mtx<T, const N: usize, const M: usize> = [[T; M]; N];

#[derive(Clone, Copy, Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    data: Mtx<T, N, M>,
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new() -> Self
    where
        Mtx<T, N, M>: Default,
    {
        Self {
            data: Default::default(),
        }
    }

    pub fn from(data: Mtx<T, N, M>) -> Self {
        Self { data }
    }

    pub fn trans(&self) -> Matrix<T, M, N>
    where
        Mtx<T, M, N>: Default,
        T: Copy,
    {
        let mut trans: Matrix<T, M, N> = Matrix::new();
        for j in 0..M {
            for i in 0..N {
                trans.data[j][i] = self.data[i][j];
            }
        }
        trans
    }

    pub fn diag(iden_value: T) -> Matrix<T, N, M>
    where
        Mtx<T, N, M>: Default,
        T: Copy + Default,
    {
        let mut iden: Matrix<T, N, M> = Matrix::new();

        for i in 0..N {
            for j in 0..M {
                if i == j {
                    iden.data[i][j] = iden_value;
                }
            }
        }
        iden
    }

    //returns a copy of the given matrix
    //with the given row and col removed.
    pub fn sub_matrix<const Q: usize, const R: usize>(
        &self,
        row: usize,
        col: usize,
    ) -> Matrix<T, Q, R>
    where
        Mtx<T, Q, R>: Default,
        T: Copy,
    {
        let mut sub_matrix: Matrix<T, Q, R> = Matrix::new();
        let mut q = 0;
        let mut r = 0;

        for i in 0..N {
            if i != row {
                for j in 0..M {
                    if j != col {
                        sub_matrix.data[q][r] = self.data[i][j];
                        r += 1;
                    }
                }
                q += 1;
                r = 0;
            }
        }

        sub_matrix
    }
}

//---------------------------OverLoad---------------------------
impl<T: ApproximateEq + PartialEq, const N: usize, const M: usize> PartialEq for Matrix<T, N, M> {
    fn eq(&self, other: &Self) -> bool {
        let mut res = false;
        for i in 0..N {
            for j in 0..M {
                res = self.data[i][j].approx_eq_low(&other.data[i][j]);
                if !res {
                    break;
                }
            }
        }
        res
    }
}

impl<T: Copy, const N: usize, const M: usize> Add for Matrix<T, N, M>
where
    T: Add<Output = T>,
    Mtx<T, N, M>: Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res: Matrix<T, N, M> = Matrix::new();

        for i in 0..N {
            for j in 0..M {
                res.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        res
    }
}

impl<T: Copy, const N: usize, const M: usize> Sub for Matrix<T, N, M>
where
    T: Add<Output = T>,
    Mtx<T, N, M>: Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res: Matrix<T, N, M> = Matrix::new();

        for i in 0..N {
            for j in 0..M {
                res.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        res
    }
}

impl<T: Copy, const N: usize, const M: usize, const P: usize> Mul<Matrix<T, M, P>>
    for Matrix<T, N, M>
where
    T: Mul<Output = T> + Add<Output = T>,
    Mtx<T, N, P>: Default,
{
    type Output = Matrix<T, N, P>;

    fn mul(self, rhs: Matrix<T, M, P>) -> Self::Output {
        let mut res: Matrix<T, N, P> = Matrix::new();

        for i in 0..N {
            for j in 0..P {
                for k in 0..M {
                    res.data[i][j] = res.data[i][j] + self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        res
    }
}

//Scalar
impl<T: Copy, const N: usize, const M: usize> Mul<T> for Matrix<T, N, M>
where
    T: Mul<Output = T>,
    Mtx<T, N, M>: Default,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut res: Matrix<T, N, M> = Matrix::new();

        for i in 0..N {
            for j in 0..M {
                res.data[i][j] = self.data[i][j] * rhs;
            }
        }

        res
    }
}

//Scalar
impl<T: Copy, const N: usize, const M: usize> Div<T> for Matrix<T, N, M>
where
    T: Div<Output = T>,
    Mtx<T, N, M>: Default,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut res: Matrix<T, N, M> = Matrix::new();

        for i in 0..N {
            for j in 0..M {
                res.data[i][j] = self.data[i][j] / rhs;
            }
        }

        res
    }
}
