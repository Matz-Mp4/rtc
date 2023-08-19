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
    fn color_with_an_intersection_behind_the_ray() {
        let mut world = World::default();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, 0.75),
            Vector::new_vec3D(0.0, 0.0, -1.0),
        );
        let mut outer = Object::new_sphere();
        outer.material.ambient = 1.0;
        let mut inner = Object::new_sphere();
        inner.material.ambient = 1.0;
        let color = world.color_at(&ray);
        world.push_object(outer);
        world.push_object(inner);
        let expected = inner.material.color;
        assert_eq!(expected, color);
    }
}
