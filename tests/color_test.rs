#[cfg(test)]
mod scalar_operations {

    use rtc::color::Color;
    #[test]
    fn mul() {
        let c = Color::new(0.2, 0.3, 0.4);
        let res = Color::new(0.4, 0.6, 0.8);

        assert_eq!(res, c * 2.0);
    }
}

#[cfg(test)]
mod primitive_operations {
    use rtc::color::Color;

    #[test]
    fn add() {
        let c1 = Color::new(0.9, 0.6, 0.75);

        let c2 = Color::new(0.7, 0.1, 0.25);

        let res = c1 + c2;
        let expected = Color::new(1.6, 0.7, 1.0);

        assert_eq!(res, expected);
    }

    #[test]
    fn mul() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let res = c1 * c2;
        let expected = Color::new(0.9, 0.2, 0.04);

        assert_eq!(res, expected);
    }
}
