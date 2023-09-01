#[cfg(test)]
mod inter_test {
    use rtc::intersection::Computations;
    use rtc::transformation::translation;
    use rtc::Intersection;
    use rtc::Intersections;
    use rtc::Object;
    use rtc::Point;
    use rtc::Ray;
    use rtc::Vector;

    #[test]
    fn all_inter_are_positive() {
        let object = Object::new_sphere();
        let i1 = Intersection::new(1.0, &object);
        let i2 = Intersection::new(2.0, &object);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(Some(&i1), inters.hit());
    }

    #[test]
    fn when_some_inter_are_negative() {
        let object = Object::new_sphere();
        let i1 = Intersection::new(-1.0, &object);
        let i2 = Intersection::new(1.0, &object);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(Some(&i2), inters.hit());
    }

    #[test]
    fn all_inter_are_negative() {
        let object = Object::new_sphere();
        let i1 = Intersection::new(-1.0, &object);
        let i2 = Intersection::new(-2.0, &object);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(None, inters.hit());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative() {
        let object = Object::new_sphere();
        let i1 = Intersection::new(5.0, &object);
        let i2 = Intersection::new(7.0, &object);
        let i3 = Intersection::new(-3.0, &object);
        let i4 = Intersection::new(2.0, &object);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);
        inters.add(i3);
        inters.add(i4);

        assert_eq!(Some(&i4), inters.hit());
    }
    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );

        let mut object = Object::new_sphere();
        object.set_transformation(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &object);
        let comp = i.prepare_computation(&ray);
        let epsilon = 1.0e-6;

        let expected = true;
        let over_point_z = *comp.over_point.get(2).unwrap();
        let point_z = *comp.point.get(2).unwrap();

        let res = over_point_z < -epsilon / 2.0 && point_z > over_point_z;

        assert_eq!(expected, res);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let sqrt2 = f64::sqrt(2.0);
        let half_sqrt2 = sqrt2 / 2.0;
        let ray = Ray::new(
            Point::new_point3D(0.0, 1.0, -1.0),
            Vector::new_vec3D(0.0, -half_sqrt2, half_sqrt2),
        );

        let object = Object::new_plane();
        let i = Intersection::new(sqrt2, &object);
        let comp = i.prepare_computation(&ray);

        let expected = Vector::new_vec3D(0.0, half_sqrt2, half_sqrt2);
        assert_eq!(expected, comp.reflectv);
    }
}
