#[cfg(test)]
mod world_test {
    use rtc::{
        color::Color, intersection::Computations, transformation::translation, Intersection,
        Intersections, Light, Object, Point, Ray, Vector, World,
    };

    #[test]
    fn shading_intersection() {
        let world = World::default_test();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );
        let inter = Intersection::new(4.0, world.get_object(0).unwrap());
        let comps = inter.prepare_computation(&ray);
        let color = world.shade_hit(&comps, world.reflection_limit);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut world = World::default_test();
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
        let color = world.shade_hit(&comps, world.reflection_limit);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
    }

    #[test]
    fn color_when_ray_misses() {
        let world = World::default_test();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 1.0, 0.0),
        );
        let color = world.color_at(&ray, world.reflection_limit);
        let expected = Color::black();

        assert_eq!(expected, color);
    }

    #[test]
    fn color_when_ray_hits() {
        let world = World::default_test();
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );
        let color = world.color_at(&ray, world.reflection_limit);
        let expected = Color::new(0.38066, 0.47583, 0.2855);

        assert_eq!(expected, color);
    }

    #[test]
    fn there_is_no_shadow() {
        let world = World::default_test();
        let p = Point::new_point3D(0.0, 10.0, 0.0);
        let expected = false;

        assert_eq!(expected, world.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let world = World::default_test();
        let p = Point::new_point3D(10.0, -10.0, 10.0);
        let expected = true;

        assert_eq!(expected, world.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let world = World::default_test();
        let p = Point::new_point3D(-20.0, 20.0, -20.0);
        let expected = false;

        assert_eq!(expected, world.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let world = World::default_test();
        let p = Point::new_point3D(-2.0, 2.0, -2.0);
        let expected = false;

        assert_eq!(expected, world.is_shadowed(&p));
    }

    #[test]
    fn reflected_color() {
        let mut world = World::default_test();

        let mut object = Object::new_plane();
        object.material.reflective = 0.5;
        object.set_transformation(translation(0.0, -1.0, 0.0));

        let sqrt2 = f64::sqrt(2.0);
        let half_sqrt2 = sqrt2 / 2.0;
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -3.0),
            Vector::new_vec3D(0.0, -half_sqrt2, half_sqrt2),
        );

        let i = Intersection::new(sqrt2, &object);
        world.push_object(object);
        let comps = i.prepare_computation(&ray);
        let result = world.reflected_color(&comps, world.reflection_limit);

        let expected = Color::new(0.19032, 0.2379, 0.14274);

        assert_eq!(expected, result);
    }

    #[test]
    fn reflected_color_maximum_recursive_depth() {
        let mut world = World::default_test();
        world.reflection_limit = 0;

        let mut object = Object::new_plane();
        object.material.reflective = 0.5;
        object.set_transformation(translation(0.0, -1.0, 0.0));

        let sqrt2 = f64::sqrt(2.0);
        let half_sqrt2 = sqrt2 / 2.0;
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -3.0),
            Vector::new_vec3D(0.0, -half_sqrt2, half_sqrt2),
        );

        let i = Intersection::new(sqrt2, &object);
        world.push_object(object);
        let comps = i.prepare_computation(&ray);
        let result = world.reflected_color(&comps, world.reflection_limit);

        let expected = Color::black();

        assert_eq!(expected, result);
    }

    #[test]
    fn refracted_color_at_maximum_recursive_depth() {
        let mut world = World::default_test();
        world.reflection_limit = 0;

        let mut temp = world.clone();
        let ob1 = temp.get_mut_object(0).unwrap();
        ob1.material.transparency = 1.0;
        ob1.material.refractive_index = 1.5;

        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -5.0),
            Vector::new_vec3D(0.0, 0.0, 1.0),
        );
        let mut inters = Intersections::new();
        inters.add(Intersection::new(4.0, &ob1));
        inters.add(Intersection::new(6.0, &ob1));

        let comp = Computations::prepare_computation(&inters, 0, &ray);
        let color = world.refracted_color(&comp, world.reflection_limit);
        let expected = Color::black();

        assert_eq!(expected, color);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut world = World::default_test();
        world.reflection_limit = 5;
        let half_sqrt2 = 2.0f64.sqrt() / 2.0;

        let mut temp = world.clone();
        let ob1 = temp.get_mut_object(0).unwrap();
        ob1.material.transparency = 1.0;
        ob1.material.refractive_index = 1.5;

        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, half_sqrt2),
            Vector::new_vec3D(0.0, 1.0, 0.0),
        );
        let mut inters = Intersections::new();
        inters.add(Intersection::new(-half_sqrt2, &ob1));
        inters.add(Intersection::new(half_sqrt2, &ob1));

        let comp = Computations::prepare_computation(&inters, 1, &ray);
        let color = world.refracted_color(&comp, world.reflection_limit);
        let expected = Color::black();

        assert_eq!(expected, color);
    }

    fn refracted_color_with_refracted_ray() {
        let mut world = World::default_test();
        world.reflection_limit = 5;

        let mut temp = world.clone();
        let ob1 = temp.get_mut_object(0).unwrap();
        ob1.material.ambient = 1.0;

        let mut another_temp = world.clone();
        let ob2 = another_temp.get_mut_object(1).unwrap();
        ob2.material.transparency = 1.0;
        ob2.material.refractive_index = 1.5;

        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, 0.1),
            Vector::new_vec3D(0.0, 1.0, 0.0),
        );
        let mut inters = Intersections::new();
        inters.add(Intersection::new(-0.9899, &ob1));
        inters.add(Intersection::new(-0.4899, &ob2));
        inters.add(Intersection::new(0.4899, &ob2));
        inters.add(Intersection::new(0.9899, &ob1));

        let comp = Computations::prepare_computation(&inters, 2, &ray);
        let color = world.refracted_color(&comp, world.reflection_limit);
        let expected = Color::new(0.0, 0.99888, 0.04725);

        assert_eq!(expected, color);
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut world = World::default_test();
        let half_sqrt2 = 2.0f64.sqrt() / 2.0;
        let ray = Ray::new(
            Point::new_point3D(0.0, 0.0, -3.0),
            Vector::new_vec3D(0.0, -half_sqrt2, half_sqrt2),
        );

        let mut floor = Object::new_plane();
        floor.set_transformation(translation(0.0,-1.0,0.0));
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;

        let mut ball = Object::new_sphere();
        ball.material.color = Color::new(1.0,0.0,0.0);
        ball.material.ambient = 0.5;
        ball.set_transformation(translation(0.0, -3.5, -0.5));

        world.push_object(floor.clone());
        world.push_object(ball.clone());



      let mut inters = Intersections::new();
        inters.add(Intersection::new(2.0f64.sqrt(), &floor));

        let comp = Computations::prepare_computation(&inters, 0, &ray);
        let color = world.refracted_color(&comp, 5);
        let expected = Color::new(0.93391, 0.69643, 0.69243);

        assert_eq!(expected, color);
    }
}
