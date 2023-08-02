#[cfg(test)]
mod sphere_test {
    use rtc::Point;
    use rtc::Ray;
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
}
