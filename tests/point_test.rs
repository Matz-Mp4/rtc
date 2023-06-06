#[cfg(test)]
mod primitive_operations {
    use rtc::tuple::point::Point;
    use rtc::tuple::vector::Vector;

    #[test]
    fn add() {
        let v = Point::new_point3(1.1, 2.2, 3.3);
        let p = Point::new_point3(1.0, 5.0, 10.0);

        let res = p + v;
        let expected = Point::new_point3(2.1, 7.2, 13.3);
        assert_eq!(res, expected);
    }

    #[test]
    fn sub() {
        let v = Point::new([1, 2, 3, 4, 5]);
        let p = Point::new([5, 4, 3, 2, 1]);

        let res = p - v;

        let expected = Vector::new([4, 2, 0, -2, -4]);
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod scalar_operations {

    use rtc::tuple::point::Point;
    #[test]
    fn mul() {
        let v = Point::new_point3(1.1, 2.2, 3.3);

        let res = v * 10.0;
        let expected = Point::new_point3(11.0, 22.0, 33.0);
        assert_eq!(res, expected);
    }
}
