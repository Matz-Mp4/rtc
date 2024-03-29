#[cfg(test)]
mod inter_test {
    use rtc::intersection::Computations;
    use rtc::transformation::scaling;
    use rtc::transformation::translation;
    use rtc::ApproximateEq;
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

    #[test]
    fn finding_n1_n2_at_various_intersection() {
        let mut object_a = Object::new_glass_sphere();
        object_a.set_transformation(scaling(2.0, 2.0, 2.0));
        object_a.material.refractive_index = 1.5;

        let mut object_b = Object::new_glass_sphere();
        object_b.set_transformation(translation(0.0, 0.0, -0.25));
        object_b.material.refractive_index = 2.0;

        let mut object_c = Object::new_glass_sphere();

        object_c.material.refractive_index = 2.0;
        object_c.set_transformation(translation(0.0, 0.0, 0.25));
        object_c.material.refractive_index = 2.5;

        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -4.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );

        let mut xs = Intersections::new();
        xs.add(Intersection::new(2.0, &object_a));
        xs.add(Intersection::new(2.75, &object_b));
        xs.add(Intersection::new(3.25, &object_c));
        xs.add(Intersection::new(4.75, &object_b));
        xs.add(Intersection::new(5.25, &object_c));
        xs.add(Intersection::new(6.0, &object_a));

        let mut comp = Computations::prepare_computation(&xs, 0, &ray);
        let mut expected = (1.0, 1.5);
        let mut result = (comp.n1, comp.n2);
        assert_eq!(expected, result);

        comp = Computations::prepare_computation(&xs, 1, &ray);
        expected = (1.5, 2.0);
        result = (comp.n1, comp.n2);
        assert_eq!(expected, result);

        comp = Computations::prepare_computation(&xs, 2, &ray);
        expected = (2.0, 2.5);
        result = (comp.n1, comp.n2);
        assert_eq!(expected, result);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let object = Object::new_glass_sphere();
        let ray = Ray {
            origin: Point::new_point3D(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            direction: Vector::new_vec3D(0.0, 1.0, 0.0),
        };

        let mut xs = Intersections::new();
        xs.add(Intersection::new(-f64::sqrt(2.0) / 2.0, &object));
        xs.add(Intersection::new(f64::sqrt(2.0) / 2.0, &object));

        let comps = Computations::prepare_computation(&xs, 1, &ray);

        assert!(comps.schlick().approx_eq_low(&1.0));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let object = Object::new_glass_sphere();
        let ray = Ray {
            origin: Point::new_point3D(0.0, 0.0, 0.0),
            direction: Vector::new_vec3D(0.0, 1.0, 0.0),
        };
        let mut xs = Intersections::new(); 
        xs.add(Intersection::new(-1.0, &object));
        xs.add(Intersection::new(1.0, &object));

        let comps = Computations::prepare_computation(&xs, 1, &ray);

        assert!(comps.schlick().approx_eq_low(&0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let object = Object::new_glass_sphere();
        let ray = Ray {
            origin: Point::new_point3D(0.0, 0.99, -2.0),
            direction: Vector::new_vec3D(0.0, 0.0, 1.0),
        };

        let mut xs = Intersections::new();
        xs.add(Intersection::new(1.8589, &object));

        let comps = Computations::prepare_computation(&xs, 0, &ray);

        assert!(comps.schlick().approx_eq_low(&0.48873));
    }
}
