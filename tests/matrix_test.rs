#[cfg(test)]
mod primitive_operations {
    use rtc::Matrix;

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
        let matx = Matrix::from([[2, 3, 3, 1], [0, 4, 3, -3], [2, -1, -1, -3], [0, -4, -3, 2]]);
        let mut expected = 8;

        assert_eq!(expected, matx.det());

        let matx2 = Matrix::from([[-3, 6, 12], [-1, 3, 5], [-1, 9, 25]]);
        expected = -42;

        assert_eq!(expected, matx2.det());
    }

    #[test]
    fn cofactor() {
        let matx = Matrix::from([[1, 2, 6], [-5, 8, -4], [2, 6, 4]]);
        let expected = 56;

        assert_eq!(expected, matx.cofactor::<2>(0, 0));
    }
}
