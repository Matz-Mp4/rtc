#[cfg(test)]
mod primitive_operations {
    use rtc::tuple::{point::Point, vector::Vector};
    use rtc::ApproximateEq;

    #[test]
    fn add() {
        let v = Vector::new_vec3(1.1, 2.2, 3.3);
        let p = Vector::new_vec3(1.0, 5.0, 10.0);

        let res = p + v;
        let expected = Vector::new_vec3(2.1, 7.2, 13.3);
        assert_eq!(res, expected);
    }

    #[test]
    fn sub() {
        let v = Vector::from([1, 2, 3, 4, 5]);
        let p = Vector::from([5, 4, 3, 2, 1]);

        let res = p - v;

        let expected = Vector::from([4, 2, 0, -2, -4]);
        assert_eq!(res, expected);
    }

    #[test]
    fn sub_vector_point() {
        let p = Point::new_point3(3, 2, 1);
        let v = Vector::new_vec3(5, 6, 7);

        let mut res = p - v;
        let expected1 = Point::new_point3(-2, -4, -6);
        assert_eq!(res, expected1);

        res = v - p;
        let expected2 = Point::new_point3(2, 4, 6);
        assert_eq!(res, expected2);
    }

    #[test]
    fn magnitude() {
        let mut v = Vector::new_vec3(1.0, 2.0, 3.0);
        let res = 14.0f64.sqrt();
        assert_eq!(res, v.magnitude());

        v = Vector::new_vec3(-1.0, -2.0, -3.0);
        assert_eq!(res, v.magnitude());
    }

    #[test]
    fn normalize() {
        let v = Vector::new_vec3(1.0, 2.0, 3.0);
        let temp = 14.0f64.sqrt();
        let res = Vector::new_vec3(1.0 / temp, 2.0 / temp, 3.0 / temp);
        assert_eq!(res, v.normalize());
    }

    #[test]
    fn cross_product() {
        let v = Vector::new_vec3(1, 2, 3);
        let p = Vector::new_vec3(2, 3, 4);

        let mut res = v | p;
        let expected1 = Vector::new_vec3(-1, 2, -1);
        let expected2 = Vector::new_vec3(1, -2, 1);
        assert_eq!(res, expected1);

        res = p | v;
        assert_eq!(res, expected2);
    }

    #[test]
    fn dot_product() {
        let v = Vector::new_vec3(1, 2, 3);
        let p = Vector::new_vec3(2, 3, 4);

        let res = v * p;
        let expected = 20;
        assert_eq!(res, expected);
    }

    #[test]
    fn reflection() {
        let v = Vector::new_vec3(1.0, -1.0, 0.0);
        let n = Vector::new_vec3(0.0, 1.0, 0.0);
        let res = v.reflect(&n);

        let expected = Vector::new_vec3(1.0, 1.0, 0.0);

        assert_eq!(expected, res);
    }
}

#[cfg(test)]
mod scalar_operations {

    use rtc::tuple::vector::Vector;
    #[test]
    fn mul() {
        let v = Vector::new_vec3(1.1, 2.2, 3.3);

        let res = v * 10.0;
        let expected = Vector::new_vec3(11.0, 22.0, 33.0);
        assert_eq!(res, expected);
    }
    #[test]
    fn div() {
        let v = Vector::new_vec3(10.0, 20.0, 30.0);

        let res = v / 10.0;
        let expected = Vector::new_vec3(1.0, 2.0, 3.0);
        assert_eq!(res, expected);
    }
}
