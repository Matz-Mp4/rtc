#[cfg(test)]
mod basic_test {
    use rtc::{transformation::*, Matrix, Point, Vector};
    use std::f64::consts::PI;

    #[test]
    fn translate_point() {
        let p = Point::new_point3D(-3.0, 4.0, 5.0);
        let transform = translation(5.0, -3.0, 2.0);
        let mut expected = Point::new_point3D(2.0, 1.0, 7.0);
        let mut res = transform * p;

        assert_eq!(expected, res);

        let inv = transform.inverse();

        res = inv * p;

        expected = Point::new_point3D(-8.0, 7.0, 3.0);
        assert_eq!(expected, res);
    }

    #[test]
    fn translate_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new_vec3D(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Point::new_point3D(-4.0, 6.0, 8.0);
        let expected = Point::new_point3D(-8.0, 18.0, 32.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn scaling_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Vector::new_vec3D(-4.0, 6.0, 8.0);
        let mut expected = Vector::new_vec3D(-8.0, 18.0, 32.0);

        assert_eq!(expected, transform * v);

        let inv = transform.inverse();
        expected = Vector::new_vec3D(-2.0, 2.0, 2.0);
        let res = inv * v;

        assert_eq!(expected, res);
    }

    #[test]
    fn reflecting_point() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let expected = Point::new_point3D(-2.0, 3.0, 4.0);
        let res = transform * p;

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn rotating_point_x_axis() {
        let p = Point::new_point3D(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let mut expected = Point::new_point3D(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0);

        assert_eq!(expected, half_quarter * p);

        expected = Point::new_point3D(0.0, 0.0, 1.0);

        assert_eq!(expected, full_quarter * p);

        let inv = half_quarter.inverse();

        expected = Point::new_point3D(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);

        assert_eq!(expected, inv * p);
    }

    #[test]
    fn rotating_point_y_axis() {
        let p = Point::new_point3D(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let mut expected = Point::new_point3D(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0);

        assert_eq!(expected, half_quarter * p);

        expected = Point::new_point3D(1.0, 0.0, 0.0);

        assert_eq!(expected, full_quarter * p);
    }

    #[test]
    fn rotating_point_z_axis() {
        let p = Point::new_point3D(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let mut expected = Point::new_point3D(-2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0);

        assert_eq!(expected, half_quarter * p);

        expected = Point::new_point3D(-1.0, 0.0, 0.0);

        assert_eq!(expected, full_quarter * p);
    }

    #[test]
    fn shearing_x_in_y() {
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let expected = Point::new_point3D(5.0, 3.0, 4.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn shearing_x_in_z() {
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let expected = Point::new_point3D(6.0, 3.0, 4.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn shearing_y_in_x() {
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let expected = Point::new_point3D(2.0, 5.0, 4.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn shearing_y_in_z() {
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let expected = Point::new_point3D(2.0, 7.0, 4.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn shearing_z_in_x() {
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let expected = Point::new_point3D(2.0, 3.0, 6.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn shearing_z_in_y() {
        let p = Point::new_point3D(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let expected = Point::new_point3D(2.0, 3.0, 7.0);

        assert_eq!(expected, transform * p);
    }

    #[test]
    fn default_view_transform() {
        let from = Point::new_point3D(0.0, 0.0, 0.0);
        let to = Point::new_point3D(0.0, 0.0, -1.0);
        let up = Vector::new_vec3D(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(Matrix::iden(), t);
    }

    #[test]
    fn view_transformation_matrix_looking_z_direction() {
        let from = Point::new_point3D(0.0, 0.0, 0.0);
        let to = Point::new_point3D(0.0, 0.0, 1.0);
        let up = Vector::new_vec3D(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);
        let expected = scaling(-1.0, 1.0, -1.0);

        assert_eq!(expected, t);
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Point::new_point3D(0.0, 0.0, 0.0);
        let to = Point::new_point3D(0.0, 0.0, 1.0);
        let up = Vector::new_vec3D(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        let expected = Matrix::from([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(expected, t);
    }
}
