#[cfg(test)]
mod ray_test {

    use rtc::transformation::*;
    use rtc::Object;
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

    #[test]
    fn translating_a_ray() {
        let mut point = Point::new_point3D(1.0, 2.0, 3.0);
        let vector = Vector::new_vec3D(0.0, 1.0, 0.0);
        let transformation = translation(3.0, 4.0, 5.0);

        let ray = Ray::new(point, vector);
        let result = ray.transform(&transformation);

        point = Point::new_point3D(4.0, 6.0, 8.0);

        let expected = Ray::new(point, vector);
        assert_eq!(expected, result);
    }

    #[test]
    fn scaling_a_ray() {
        let mut point = Point::new_point3D(1.0, 2.0, 3.0);
        let mut vector = Vector::new_vec3D(0.0, 1.0, 0.0);
        let transformation = scaling(2.0, 3.0, 4.0);

        let ray = Ray::new(point, vector);
        let result = ray.transform(&transformation);

        point = Point::new_point3D(2.0, 6.0, 12.0);
        vector = Vector::new_vec3D(0.0, 3.0, 0.0);

        let expected = Ray::new(point, vector);
        assert_eq!(expected, result);
    }

    #[test]
    fn interesecting_a_scaled_sphere() {
        let point = Point::new_point3D(0.0, 0.0, -5.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);
        let ray = Ray::new(point, vector);
        let mut object = Object::new_sphere();
        object.set_transformation(scaling(2.0, 2.0, 2.0));

        if let Some((t1, t2)) = object.intersects(&ray) {
            assert_eq!(3.0, t1);
            assert_eq!(7.0, t2);
        } 
    }

    #[test]
    fn interesecting_a_translated_sphere() {
        let point = Point::new_point3D(0.0, 0.0, -5.0);
        let vector = Vector::new_vec3D(0.0, 0.0, 1.0);
        let ray = Ray::new(point, vector);
        let mut object = Object::new_sphere();
        object.set_transformation(translation(5.0, 2.0, 2.0));

        let result = object.intersects(&ray);
        assert_eq!(None, result);
         
    }
}
