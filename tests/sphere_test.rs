#[cfg(test)]
mod sphere_test {
    use std::f64::consts::PI;

    use rtc::transformation::*;
    use rtc::Object;
    use rtc::Point;
    use rtc::Ray;
    use rtc::Shape;
    use rtc::Sphere;
    use rtc::Vector;

    #[test]
    fn intertesects_at_two_points() {
        let point = Point::new_point3D(0.0, 0.0, -5.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);

        let ray = Ray::new(point, vector);

        if let Some((t1, t2)) = Sphere::intersect(&ray) {
            assert_eq!(4.0, t1);
            assert_eq!(6.0, t2);
        }
    }

    #[test]
    fn intertesects_at_tangent() {
        let point = Point::new_point3D(0.0, 1.0, -5.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);

        let ray = Ray::new(point, vector);

        if let Some((t1, t2)) = Sphere::intersect(&ray) {
            assert_eq!(5.0, t1);
            assert_eq!(5.0, t2);
        }
    }

    #[test]
    fn misses() {
        let point = Point::new_point3D(0.0, 2.0, -5.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);

        let ray = Ray::new(point, vector);

        let t = Sphere::intersect(&ray);
        assert_eq!(None, t);
    }

    #[test]
    fn sphere_is_inside_ray() {
        let point = Point::new_point3D(0.0, 0.0, 0.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);

        let ray = Ray::new(point, vector);

        if let Some((t1, t2)) = Sphere::intersect(&ray) {
            assert_eq!(-1.0, t1);
            assert_eq!(1.0, t2);
        }
    }

    #[test]
    fn sphere_is_behind_ray() {
        let point = Point::new_point3D(0.0, 0.0, 5.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);

        let ray = Ray::new(point, vector);

        if let Some((t1, t2)) = Sphere::intersect(&ray) {
            assert_eq!(-6.0, t1);
            assert_eq!(-4.0, t2);
        }
    }

    #[test]
    fn normal_at_a_nonaxial_point() {
        let value = 3.0f64.sqrt() / 3.0;
        let point = Point::new_point3D(value, value, value);
        let sphere = Object::new_sphere();

        let n = sphere.normal_at(&point);

        let expected = Vector::new_vec3D(value, value, value);

        assert_eq!(expected, n);
    }

    #[test]
    fn normal_is_a_normilized_vector() {
        let value = 3.0f64.sqrt() / 3.0;
        let point = Point::new_point3D(value, value, value);
        let sphere = Object::new_sphere();

        let n = sphere.normal_at(&point);

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let mut object = Object::new_sphere();
        object.set_transformation(translation(0.0, 1.0, 0.0));
        let point = Point::new_point3D(0.0, 1.70711, -0.70711);
        let n = object.normal_at(&point);
        let expected = Vector::new_vec3D(0.0, 0.70711, -0.70711);
        assert_eq!(expected, n);
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let mut object = Object::new_sphere();

        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        object.set_transformation(m);

        let point = Point::new_point3D(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
        let n = object.normal_at(&point);
        let expected = Vector::new_vec3D(0.0, 0.97014, -0.24254);

        assert_eq!(expected, n);
    }
}
