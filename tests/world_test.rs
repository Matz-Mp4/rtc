#[cfg(test)]
mod world_test {
    use rtc::{color::Color, Intersection, Light, Object, Point, Ray, Vector, World};

    #[test]
    fn shading_intersection() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );
        let inter = Intersection::new(4.0, world.get_object(0).unwrap());
        let comps = inter.prepare_computation(&ray);
        let color = world.shade_hit(&comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut world = World::default();
        world.light = Light::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new_point3D(0.0, 0.25, 0.0),
        );
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, 0.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );
        let inter = Intersection::new(0.5, world.get_object(1).unwrap());
        let comps = inter.prepare_computation(&ray);
        let color = world.shade_hit(&comps);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
    }

    #[test]
    fn color_when_ray_misses() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 1.0, 0.0),
        );
        let color = world.color_at(&ray);
        let expected = Color::black();

        assert_eq!(expected, color);
    }

    #[test]
    fn color_when_ray_hits() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );
        let color = world.color_at(&ray);
        let expected = Color::new(0.38066, 0.47583, 0.2855);

        assert_eq!(expected, color);
    }

    #[test]
    fn there_is_no_shadow() {
        let world = World::default();
        let p = Point::new_point3D(0.0, 10.0, 0.0);
        let expected = false;

        assert_eq!(expected, world.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let world = World::default();
        let p = Point::new_point3D(10.0, -10.0, 10.0);
        let expected = true;

        assert_eq!(expected, world.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let world = World::default();
        let p = Point::new_point3D(-20.0, 20.0, -20.0);
        let expected = false;

        assert_eq!(expected, world.is_shadowed(&p));
    }


    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let world = World::default();
        let p = Point::new_point3D(-2.0, 2.0, -2.0);
        let expected = false;

        assert_eq!(expected, world.is_shadowed(&p));
    }
}
