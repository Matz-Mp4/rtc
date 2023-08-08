#[cfg(test)]
mod inter_test {
    use rtc::Intersection;
    use rtc::Intersections;
    use rtc::Object;
    use rtc::Shape;
    use rtc::Sphere;

    #[test]
    fn all_inter_are_positive() {
        let s = Shape::Sphere;
        let i1 = Intersection::new(1.0, Object::new(s));
        let i2 = Intersection::new(2.0, Object::new(s));
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(Some(&i1), inters.hit());
    }


    #[test]
    fn when_some_inter_are_negative() {
        let s = Shape::Sphere;
        let i1 = Intersection::new(-1.0, Object::new(s));
        let i2 = Intersection::new(1.0, Object::new(s));
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);

        assert_eq!(Some(&i1), inters.hit());
    }

}
