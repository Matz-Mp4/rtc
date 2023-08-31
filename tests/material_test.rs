#[cfg(test)]
mod material_test {
    use rtc::{color::Color, Light, Material, Object, Point, Vector};

    #[test]
    fn eye_opposite_surface() {
        let position = Point::new_point3D(0.0, 0.0, 0.0);
        let eyev = Vector::new_vec3D(0.0, 0.0, -1.0);
        let normalv = Vector::new_vec3D(0.0, 0.0, -1.0);
        let light = Light::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new_point3D(0.0, 0.0, -10.0),
        );

        let material = Material::default();

        let res = material.lightning(
            &Object::new_sphere(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );
        let expected = Color::new(1.9, 1.9, 1.9);

        assert_eq!(expected, res);
    }

    #[test]
    fn eye_between_light_surface() {
        //Eye offset 45
        let position = Point::new_point3D(0.0, 0.0, 0.0);
        let eyev = Vector::new_vec3D(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
        let normalv = Vector::new_vec3D(0.0, 0.0, -1.0);
        let light = Light::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new_point3D(0.0, 0.0, -10.0),
        );

        let material = Material::default();

        let res = material.lightning(
            &Object::new_sphere(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );
        let expected = Color::new(1.0, 1.0, 1.0);

        assert_eq!(expected, res);
    }

    #[test]
    fn lightning_with_the_light_behind_surface() {
        //Eye offset 45
        let position = Point::new_point3D(0.0, 0.0, 0.0);
        let eyev = Vector::new_vec3D(0.0, 0.0, -0.1);
        let normalv = Vector::new_vec3D(0.0, 0.0, -1.0);
        let light = Light::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new_point3D(0.0, 0.0, 10.0),
        );

        let material = Material::default();

        let res = material.lightning(
            &Object::new_sphere(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );
        let expected = Color::new(0.1, 0.1, 0.1);

        assert_eq!(expected, res);
    }
}
