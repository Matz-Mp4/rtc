#[cfg(test)]
mod primitive_operations {
    use rtc::{Matrix, Point, Vector};

    #[test]
    fn eq_and_not_eq() {
        let matx1 = Matrix::from([[1, 2, 3], [3, 4, 5], [7, 8, 9]]);
        let matx2 = Matrix::from([[1, 2, 3], [3, 4, 5], [7, 8, 9]]);
        let matx3 = Matrix::from([[4, 5, 3], [1, 4, 5], [0, 8, 9]]);

        assert_eq!(matx1, matx2);
        assert_ne!(matx1, matx3);
    }

    #[test]
    fn mul() {
        let matx1 = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let matx2 = Matrix::from([[-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]]);
        let expected = Matrix::from([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
        ]);

        let res = matx1 * matx2;

        assert_eq!(expected, res);
    }

    #[test]
    fn iden() {
        let matx1 = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let iden = Matrix::iden();
        let res = matx1 * iden;

        assert_eq!(matx1, res);
    }

    #[test]
    fn trans() {
        let matx1 = Matrix::from([[0, 9, 3, 0], [9, 8, 0, 8], [1, 8, 5, 3], [0, 0, 5, 8]]);
        let expected = Matrix::from([[0, 9, 1, 0], [9, 8, 8, 0], [3, 0, 5, 5], [0, 8, 3, 8]]);
        let res = matx1.trans();

        assert_eq!(expected, res);
    }

    #[test]
    fn sub_matrix() {
        let matx1 = Matrix::from([[1, 5, 0], [-3, 2, 7], [0, 6, -3]]);
        let expected = Matrix::from([[-3, 2], [0, 6]]);
        let res: Matrix<i32, 2, 2> = matx1.sub_matrix(0, 2);

        assert_eq!(expected, res);
    }
    #[test]
    fn row_reduce() {
        let mut matx1 = Matrix::from([
            [1.0, 2.0, -1.0, -4.0],
            [2.0, 3.0, -1.0, -11.0],
            [-2.0, 0.0, -3.0, 22.0],
        ]);

        let expected = Matrix::from([
            [1.0, 0.0, 0.0, -8.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, -2.0],
        ]);

        assert_eq!(expected, matx1.row_reduce());
    }

    #[test]
    fn det() {
        let matx = Matrix::from([
            [2.0, 3.0, 3.0, 1.0],
            [0.0, 4.0, 3.0, -3.0],
            [2.0, -1.0, -1.0, -3.0],
            [0.0, -4.0, -3.0, 2.0],
        ]);
        let mut expected = 8.0;

        assert_eq!(expected, matx.det());

        let matx2 = Matrix::from([[-3.0, 6.0, 12.0], [-1.0, 3.0, 5.0], [-1.0, 9.0, 25.0]]);
        expected = -42.0;

        assert_eq!(expected, matx2.det());
    }

    #[test]
    fn cofactor() {
        let matx = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        let expected = 56.0;

        assert_eq!(expected, matx.cofactor::<2>(0, 0));
    }
    #[test]
    fn inverse() {
        let mut matx = Matrix::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let mut expected = Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(expected, matx.inverse::<3>());

        matx = Matrix::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        expected = Matrix::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(expected, matx.inverse::<3>());

        matx = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let matx2 = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let matx3 = matx * matx2;
        expected = matx;
        let res = matx3 * matx2.inverse::<3>();

        assert_eq!(expected, res);
    }

    #[test]
    fn mul_vector() {
        let matx = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let vec: Vector<f64, 4> = Vector::from([1.0, 2.0, 3.0, 1.0]);
        let res = matx * vec;

        let expected = Vector::from([18.0, 24.0, 33.0, 1.0]);

        assert_eq!(expected, res);

        let matx2 = Matrix::from([[1.0, 2.0, -3.0], [2.0, 9.0, 0.0], [6.0, -1.0, -2.0]]);
        let vec2: Vector<f64, 3> = Vector::from([2.0, 3.0, -1.0]);
        let res2 = matx2 * vec2;
        let expected2: Vector<f64, 3> = Vector::from([11.0, 31.0, 11.0]);

        assert_eq!(expected2, res2);
    }

    #[test]
    fn mul_point() {
        let matx = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let vec: Point<f64, 4> = Point::from([1.0, 2.0, 3.0, 1.0]);
        let res = matx * vec;

        let expected = Point::from([18.0, 24.0, 33.0, 1.0]);

        assert_eq!(expected, res);
    }
}
