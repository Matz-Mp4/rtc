#[cfg(test)]
mod inter_test {
    use rtc::Intersection;
    use rtc::Intersections;
    use rtc::Object;
    use rtc::Shape;
    use rtc::Sphere;

    #[test]
    fn all_inter_are_positive() {
        let i1 = Intersection::new(1.0, Object::new_sphere());
        let i2 = Intersection::new(2.0, Object::new_sphere());
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(Some(&i1), inters.hit());
    }

    #[test]
    fn when_some_inter_are_negative() {
        let i1 = Intersection::new(-1.0, Object::new_sphere());
        let i2 = Intersection::new(1.0, Object::new_sphere());
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(Some(&i2), inters.hit());
    }

    #[test]
    fn all_inter_are_negative() {
        let i1 = Intersection::new(-1.0, Object::new_sphere());
        let i2 = Intersection::new(-2.0, Object::new_sphere());
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(None, inters.hit());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative() {
        let i1 = Intersection::new(5.0, Object::new_sphere());
        let i2 = Intersection::new(7.0, Object::new_sphere());
        let i3 = Intersection::new(-3.0, Object::new_sphere());
        let i4 = Intersection::new(2.0, Object::new_sphere());
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);
        inters.add(i3);
        inters.add(i4);

        assert_eq!(Some(&i4), inters.hit());
    }

    #[test]
    fn interesecting_a_scaled_sphere() {
    }
}
