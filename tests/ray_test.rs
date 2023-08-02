#[cfg(test)]
mod ray_test {

    use rtc::Point;
    use rtc::Ray;
    use rtc::Vector;

    #[test]
    fn distance() {
        let point = Point::new_point3D(2.0, 3.0, 4.0);
        let vector = Vector::new_vec3D(1.0, 0.0, 0.0);

        let ray = Ray::new(point, vector);

        assert_eq!(Point::new_point3D(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Point::new_point3D(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Point::new_point3D(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Point::new_point3D(4.5, 3.0, 4.0), ray.position(2.5));
    }
}
