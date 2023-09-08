#[cfg(test)]
mod ray_test {

    use rtc::color::Color;
    use rtc::transformation::rotation_y;
    use rtc::transformation::translation;
    use rtc::transformation::view_transform;
    use rtc::ApproximateEq;
    use rtc::Camera;
    use rtc::Point;
    use rtc::Ray;
    use rtc::Vector;
    use rtc::World;
    use std::f64::consts::PI;

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        let expected = 0.01;

        assert_eq!(true, expected.approx_eq(&c.pixel_size));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        let expected = 0.01;

        assert_eq!(true, expected.approx_eq(&c.pixel_size));
    }

    #[test]
    fn ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        let expected = Ray::new(
            Point::new_point3D(0.0, 0.0, 0.0),
            Vector::new_vec3D(0.0, 0.0, -1.0),
        );

        assert_eq!(expected, r);
    }

    #[test]
    fn ray_through_the_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        let expected = Ray::new(
            Point::new_point3D(0.0, 0.0, 0.0),
            Vector::new_vec3D(0.66519, 0.33259, -0.66851),
        );

        assert_eq!(expected, r);
    }

    #[test]
    fn ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
        c.with_transformation(&transform);
        let r = c.ray_for_pixel(100, 50);
        let expected = Ray::new(
            Point::new_point3D(0.0, 2.0, -5.0),
            Vector::new_vec3D(2.0f64.sqrt() / 2.0, 0.0, -2.0f64.sqrt() / 2.0),
        );

        assert_eq!(expected, r);
    }

    #[test]
    fn render_a_world() {
        let w = World::default_test();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new_point3D(0.0, 0.0, -5.0);
        let to = Point::new_point3D(0.0, 0.0, 0.0);
        let up = Vector::new_vec3D(0.0, 1.0, 0.0);
        c.with_transformation(&view_transform(&from, &to, &up));
        let image = c.render(&w);
        let expected = Color::new(0.38066, 0.47583, 0.2855);
        assert_eq!(expected, image[5][5]);
    }
}
