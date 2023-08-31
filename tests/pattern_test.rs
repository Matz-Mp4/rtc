#[cfg(test)]
mod pattern_test {

    use rtc::{
        color::Color,
        transformation::{scaling, translation},
        Object, Pattern, PatternType, Point,
    };

    mod striped {

        use rtc::color::Color;
        use rtc::pattern::Pattern;
        use rtc::pattern::PatternType;
        use rtc::pattern::Striped;
        use rtc::transformation::scaling;
        use rtc::transformation::translation;
        use rtc::Light;
        use rtc::Object;
        use rtc::Point;
        use rtc::Vector;

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
            let mut object = Object::new_sphere();
            object.material.set_ambient(1.0);
            object.material.set_diffuse(0.0);
            object.material.set_specular(0.0);
            object
                .material
                .set_pattern(Pattern::with_type(PatternType::striped_pattern(
                    Color::white(),
                    Color::black(),
                )));

            let eyev = Vector::new_vec3D(0.0, 0.0, -1.0);
            let normalv = Vector::new_vec3D(0.0, 0.0, -1.0);
            let light = Light::new(Color::white(), Point::new_point3D(0.0, 0.0, -10.0));

            let c1 = object.material.lightning(
                &object,
                &light,
                &Point::new_point3D(0.9, 0.0, 0.0),
                &eyev,
                &normalv,
                false,
            );
            let c2 = object.material.lightning(
                &object,
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

        #[test]
        fn stripes_with_object_transformed() {
            let mut object = Object::new_sphere();
            object.set_transformation(scaling(2.0, 2.0, 2.0));
            object
                .material
                .set_pattern(Pattern::with_type(PatternType::striped_pattern(
                    Color::white(),
                    Color::black(),
                )));

            let c = object
                .pattern_at_object(&Point::new_point3D(1.5, 0.0, 0.0))
                .unwrap();
            let expected = Color::white();

            assert_eq!(expected, c);
        }

        #[test]
        fn stripes_with_pattern_transformed() {
            let mut object = Object::new_sphere();
            object
                .material
                .set_pattern(Pattern::with_type(PatternType::striped_pattern(
                    Color::white(),
                    Color::black(),
                )));

            object
                .material
                .pattern
                .set_transformation(scaling(2.0, 2.0, 2.0));

            let c = object
                .pattern_at_object(&Point::new_point3D(1.5, 0.0, 0.0))
                .unwrap();
            let expected = Color::white();

            assert_eq!(expected, c);
        }

        #[test]
        fn stripes_with_pattern_object_transformed() {
            let mut object = Object::new_sphere();
            object.set_transformation(scaling(2.0, 2.0, 2.0));
            object
                .material
                .set_pattern(Pattern::with_type(PatternType::striped_pattern(
                    Color::white(),
                    Color::black(),
                )));

            object
                .material
                .pattern
                .set_transformation(translation(0.5, 0.0, 0.0));

            let c = object
                .pattern_at_object(&Point::new_point3D(2.5, 0.0, 0.0))
                .unwrap();
            let expected = Color::white();

            assert_eq!(expected, c);
        }
    }

    mod gradient {
        use rtc::{color::Color, pattern::Gradient, Point};

        #[test]
        fn gradient_linearly_interpolates_colors() {
            let pattern = Gradient::new(Color::white(), Color::black());
            let mut expected = Color::white();

            assert_eq!(
                expected,
                pattern.gradient_at(&Point::new_point3D(0.0, 0.0, 0.0))
            );

            expected = Color::new(0.75, 0.75, 0.75);
            assert_eq!(
                expected,
                pattern.gradient_at(&Point::new_point3D(0.25, 0.0, 0.0))
            );

            expected = Color::new(0.5, 0.5, 0.5);
            assert_eq!(
                expected,
                pattern.gradient_at(&Point::new_point3D(0.5, 2.0, 0.0))
            );

            expected = Color::new(0.25, 0.25, 0.25);
            assert_eq!(
                expected,
                pattern.gradient_at(&Point::new_point3D(0.75, 0.0, 0.0))
            );
        }
    }

    mod ring {
        use rtc::{color::Color, pattern::Ring, Point};

        #[test]
        fn ring_should_extend_in_x_and_z() {
            let pattern = Ring::new(Color::white(), Color::black());
            let mut expected = Color::white();

            assert_eq!(
                expected,
                pattern.ring_at(&Point::new_point3D(0.0, 0.0, 0.0))
            );

            expected = Color::black();
            assert_eq!(
                expected,
                pattern.ring_at(&Point::new_point3D(1.0, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.ring_at(&Point::new_point3D(0.0, 0.0, 1.0))
            );

            assert_eq!(
                expected,
                pattern.ring_at(&Point::new_point3D(0.708, 0.0, 0.708))
            );
        }
    }

    mod checker{
        use rtc::{color::Color, pattern::Checker, Point};

        #[test]
        fn checker_should_repeat_in_x() {
            let pattern = Checker::new(Color::white(), Color::black());
            let mut expected = Color::white();

            assert_eq!(
                expected,
                pattern.checker_at(&Point::new_point3D(0.0, 0.0, 0.0))
            );

            assert_eq!(
                expected,
                pattern.checker_at(&Point::new_point3D(0.99, 0.0, 0.0))
            );

            expected = Color::black();
            assert_eq!(
                expected,
                pattern.checker_at(&Point::new_point3D(1.01, 0.0, 0.0))
            );

            
        }
    }
}
