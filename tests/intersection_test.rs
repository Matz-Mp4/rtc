#[cfg(test)]
mod inter_test {
    use rtc::intersection::Computations;
    use rtc::Intersection;
    use rtc::Intersections;
    use rtc::Object;
    use rtc::Point;
    use rtc::Ray;
    use rtc::Shape;
    use rtc::Sphere;
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
    fn intersection_occurs_on_the_inside() {
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, 0.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );

        let object = Object::new_sphere();
        let i = Intersection::new(1.0, &object);
        let res = i.prepare_computation(&ray);
        let expected = Computations::new(
            1.0,
            &object,
            Point::new_point3D(0.0, 0.0, 1.0),
            Vector::new_vec3D(0.0, 0.0, -1.0),
            Vector::new_vec3D(0.0, 0.0, -1.0),
            true,
        );

        assert_eq!(expected, res);
    }
}
