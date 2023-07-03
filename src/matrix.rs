use crate::Point;
use crate::Tuple;
use crate::Vector;

use super::ApproximateEq;
use super::One;
use super::Zero;
use std::ops::{Add, Div, Mul, Neg, Sub};

type Mtx<T, const N: usize, const M: usize> = [[T; M]; N];

#[derive(Clone, Copy, Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    data: Mtx<T, N, M>,
}

impl<T: Zero + Copy, const N: usize, const M: usize> Zero for Mtx<T, N, M> {
    fn zero() -> Self {
        let data: Mtx<T, N, M> = [[Zero::zero(); M]; N];
        data
    }
}

impl<T: Sized, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new() -> Self
    where
        T: Zero + Copy + Sized,
    {
        let data: Mtx<T, N, M> = Zero::zero();

        Self { data }
    }

    pub fn from(data: Mtx<T, N, M>) -> Self {
        Self { data }
    }

    pub fn get_row(&self, row: usize) -> [&T; M] {
        //why
        let mut output = [&self.data[row][0]; M];

        for i in 0..M {
            output[i] = &self.data[row][i];
        }

        output
    }

    pub fn get_col(&self, col: usize) -> [&T; N] {
        //why
        let mut output = [&self.data[0][col]; N];

        for i in 0..N {
            output[i] = &self.data[i][col];
        }
        output
    }

    pub fn row_reduce(&mut self) -> Matrix<T, N, M>
    where
        T: Zero + Copy + Sized + PartialEq + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    {
        let zero = Zero::zero();
        let mut pivot = 0;
        let mut matrix_out = self.clone();

        'outer: for r in 0..N {
            if M <= pivot {
                break;
            }
            let mut i = r;

            while matrix_out.data[i][pivot] == zero {
                i += 1;
                if i == N {
                    i = r;
                    pivot += 1;
                    if M == pivot {
                        pivot = pivot - 1;
                        break 'outer;
                    }
                }
            }
            for j in 0..N {
                let temp = matrix_out.data[r][j];
                matrix_out.data[r][j] = matrix_out.data[i][j];
                self.data[i][j] = temp;
            }

            let div = matrix_out.data[r][pivot];
            if div != zero {
                for j in 0..M {
                    matrix_out.data[i][j] = matrix_out.data[r][j] / div;
                }
            }

            for j in 0..N {
                if j != r {
                    let hold = matrix_out.data[j][pivot];
                    for k in 0..M {
                        matrix_out.data[j][k] =
                            matrix_out.data[j][k] - (hold * matrix_out.data[r][k]);
                    }
                }
            }
            pivot += 1;
        }

        matrix_out
    }

    pub fn trans(&self) -> Matrix<T, M, N>
    where
        Mtx<T, M, N>: Zero,
        T: Copy,
        T: Zero,
    {
        let mut trans: Matrix<T, M, N> = Matrix::new();
        for j in 0..M {
            for i in 0..N {
                trans.data[j][i] = self.data[i][j];
            }
        }
        trans
    }

    pub fn iden() -> Matrix<T, N, M>
    where
        T: Copy,
        T: Zero + One,
    {
        let mut iden: Matrix<T, N, M> = Matrix::new();

        for i in 0..N {
            for j in 0..M {
                if i == j {
                    iden.data[i][j] = One::one();
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
        T: Zero + Copy,
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

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Copy + Sized + Zero + Mul<Output = T> + One + Div<Output = T> + Sub<Output = T> + PartialEq,
{
    pub fn det(&self) -> T {
        let mut det = One::one();
        let mut temp_matrix = self.clone();

        for i in 0..N {
            let mut search = i;
            let max;

            if temp_matrix.data[i][i] == Zero::zero() {
                while temp_matrix.data[i][search] == Zero::zero() && search < N {
                    search += 1;
                }

                if search < N {
                    for j in 0..N {
                        let swap = temp_matrix.data[j][search];
                        temp_matrix.data[j][search] = temp_matrix.data[j][i];
                        temp_matrix.data[j][i] = swap;
                    }
                } else {
                    return Zero::zero();
                }
            }

            max = temp_matrix.data[i][i];
            det = max * det;

            for j in (i..N).rev() {
                temp_matrix.data[j][i] = temp_matrix.data[j][i] / max;
                for k in (i + 1)..N {
                    temp_matrix.data[j][k] =
                        temp_matrix.data[j][k] - (temp_matrix.data[j][i] * temp_matrix.data[i][k]);
                }
            }
        }

        det
    }

    pub fn cofactor<const Q: usize>(&self, row: usize, col: usize) -> T
    where
        T: Zero + Copy + Neg<Output = T>,
    {
        let mut cofact = Zero::zero();

        if row <= N && col <= N {
            let minor: Matrix<T, Q, Q> = self.sub_matrix(row, col);
            cofact = minor.det();

            if ((row + col) & 1) == 1 {
                cofact = -cofact;
            }
        }
        cofact
    }

    //Q = N - 1
    //Super Trash
    pub fn inverse<const Q: usize>(&self) -> Matrix<T, N, N>
    where
        T: Zero + Neg<Output = T> + Sized,
    {
        let mut output = Matrix::new();
        let det = self.det();

        for row in 0..N {
            for col in 0..N {
                let c = self.cofactor::<Q>(row, col);
                output.data[col][row] = c / det;
            }
        }

        output
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
    T: Zero,
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
    T: Zero,
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

//Matrix
impl<T: Copy, const N: usize, const M: usize, const P: usize> Mul<Matrix<T, M, P>>
    for Matrix<T, N, M>
where
    T: Mul<Output = T> + Add<Output = T>,
    T: Zero,
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

//Point
impl<T: Copy, const N: usize, const M: usize> Mul<Point<T, M>> for Matrix<T, N, M>
where
    T: Mul<Output = T> + Add<Output = T>,
    T: Zero,
{
    type Output = Point<T, N>;

    fn mul(self, rhs: Point<T, M>) -> Self::Output {
        let mut res: Point<T, N> = Point::new();

        for i in 0..N {
            let res_data = res.get_mut(i).unwrap();
            for j in 0..M {
                let rhs_data = *(rhs.get(j).unwrap());
                *res_data = *res_data + self.data[i][j] * rhs_data;
            }
        }

        res
    }
}

//Vector
impl<T: Copy, const N: usize, const M: usize> Mul<Vector<T, M>> for Matrix<T, N, M>
where
    T: Mul<Output = T> + Add<Output = T>,
    T: Zero,
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: Vector<T, M>) -> Self::Output {
        let mut res: Vector<T, N> = Vector::new();

        for i in 0..N {
            let res_data = res.get_mut(i).unwrap();
            for j in 0..M {
                let rhs_data = *(rhs.get(j).unwrap());
                *res_data = *res_data + self.data[i][j] * rhs_data;
            }
        }

        res
    }
}

//Scalar
impl<T: Copy, const N: usize, const M: usize> Mul<T> for Matrix<T, N, M>
where
    T: Mul<Output = T>,
    T: Zero,
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
    T: Zero,
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
