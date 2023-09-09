use std::f64::consts::PI;

use rtc::{
    color::Color,
    transformation::{rotation_x, rotation_y, rotation_z, scaling, translation, view_transform},
    Camera, Light, Motion, Object, Pattern, PatternType, Point, Vector, World,
};

fn main() {
    let mut wall_back = Object::new_plane()
        .with_transformation(translation(1.0, 1.0, 5.0) * rotation_x(PI / 2.0))
        .with_color(Color::black());
    /*.with_pattern(Pattern::with_type(PatternType::ring_pattern(
        Color::new(0.2, 0.2, 0.2),
        Color::black(),
    )));*/

    wall_back.material.specular = 7.0;
    wall_back.material.reflective = 1.0;
    wall_back.material.diffuse = 1.0;
    wall_back.material.shininess = 1600.0;

    let mut wall_left = Object::new_plane()
        .with_transformation(translation(-3.0, 1.0, 1.0) * rotation_z(PI / 2.0))
        .with_color(Color::black());
    wall_left.material.specular = 7.0;
    wall_left.material.reflective = 1.0;
    wall_left.material.diffuse = 1.0;
    wall_left.material.shininess = 1600.0;
    let mut wall_right = wall_left.clone();
    wall_right.set_transformation(translation(12.0, 1.0, 1.0) * rotation_z(PI / 2.0));

    let mut floor = Object::new_plane().with_pattern(Pattern::with_type(
        PatternType::checker_pattern(Color::white(), Color::black()),
    ));
    floor.material.specular = 1.0;
    floor.material.reflective = 1.0;

    let mut middle = Object::new_sphere();
    middle.set_transformation(translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::black();
    middle.material.reflective = 1.0;
    middle.material.shininess = 1600.0;
    middle.material.specular = 5.0;
    middle.material.diffuse = 1.1;
    middle.material.refractive_index = 1.0;

    let mut right = Object::new_sphere()
        .with_color(Color::black())
        .with_transformation(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5))
        .with_pattern(Pattern::with_type(PatternType::ring_pattern(
            Color::new(0.0, 0.2, 0.9),
            Color::new(0.0, 0.3, 0.7),
        )));
    right
        .material
        .pattern
        .set_transformation(scaling(0.09, 0.09, 0.03) * rotation_z(-PI / 4.0));
    right.material.reflective = 1.0;
    right.material.shininess = 1600.0;
    right.material.specular = 10.0;
    right.material.diffuse = 0.5;

    let mut left = Object::new_sphere()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_transformation(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33))
        .with_pattern(Pattern::with_type(PatternType::gradient_pattern(
            Color::blue(),
            Color::red(),
        )));
    left.material.diffuse = 1.0;
    left.material.reflective = 1.0;
    left.material.specular = 0.3;
    left.material.pattern.set_transformation(
        rotation_z(2.0 * PI / 3.0) * translation(1.0, 0.0, 0.0) * scaling(2.0, 2.0, 2.0),
    );

    let mut objects = Vec::new();
    objects.push(floor);
    objects.push(wall_left);
    /* objects.push(wall_right); */
    objects.push(wall_back);
    objects.push(middle);
    objects.push(right);
    objects.push(left);

    let light = Light::new(
        Color::white(),
         Point::new_point3D(10.0, 5.0, -10.0),
    );


    let world = World::new(light, objects, 200);
    let mut camera = Camera::new(1280, 720, PI / 3.0);
    camera.set_transformation(&view_transform(
        &Point::new_point3D(0.0, 1.5, -5.0),
        &Point::new_point3D(0.0, 1.0, 0.0),
        &Vector::new_vec3D(0.0, 1.0, 0.0),
    ));



    let canvas = camera.render(&world);
    canvas.convert_to_png("pictures/example_2.png").expect("Error");
}
