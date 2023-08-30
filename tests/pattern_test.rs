#[cfg(test)]
mod pattern_test {

    mod striped {


        use rtc::color::Color;
        use rtc::pattern::PatternType;
        use rtc::pattern::Pattern;
        use rtc::pattern::Striped;
        use rtc::Light;
        use rtc::Vector;
        use rtc::{Material, Point};

        #[test]
        fn pattern_is_constant_in_y() {
            let pattern = Striped::new(Color::white(), Color::black());
            let expected = Color::white();

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 1.0, 0.0))
            );
            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 2.0, 0.0))
            );
        }

        #[test]
        fn pattern_is_constant_in_z() {
            let pattern = Striped::new(Color::white(), Color::black());
            let expected = Color::white();

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 0.0, 1.0))
            );
            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 0.0, 2.0))
            );
        }

        #[test]
        fn pattern_is_constant_in_x() {
            let pattern = Striped::new(Color::white(), Color::black());
            let mut expected = Color::white();

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.0, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(0.9, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(-1.1, 0.0, 0.0))
            );

            expected = Color::black();

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(1.0, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(-0.1, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.stripe_at(&Point::new_point3D(-1.0, 0.0, 0.0))
            );
        }

        #[test]
        pub fn lighting_with_a_striped_pattern() {
            let mut material = Material::default();
            material.set_ambient(1.0);
            material.set_diffuse(0.0);
            material.set_specular(0.0);
            material.set_pattern(Pattern::with_type(PatternType::striped_pattern(
                Color::white(),
                Color::black(),
            )));

            let eyev = Vector::new_vec3D(0.0, 0.0, -1.0);
            let normalv = Vector::new_vec3D(0.0, 0.0, -1.0);
            let light = Light::new(Color::white(), Point::new_point3D(0.0, 0.0, -10.0));

            let c1 = material.lightning(
                &light,
                &Point::new_point3D(0.9, 0.0, 0.0),
                &eyev,
                &normalv,
                false,
            );
            let c2 = material.lightning(
                &light,
                &Point::new_point3D(1.1, 0.0, 0.0),
                &eyev,
                &normalv,
                false,
            );

            let mut expected = Color::white();

            assert_eq!(expected, c1);
            expected = Color::black();
            assert_eq!(expected, c2);
        }
    }
}
