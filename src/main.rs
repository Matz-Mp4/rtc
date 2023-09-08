use std::f64::consts::PI;

use rtc::{
    color::Color,
    transformation::{rotation_x, rotation_y, rotation_z, scaling, translation, view_transform},
    Camera, Light, Motion, Object, Pattern, PatternType, Point, Vector, World,
};

fn main() {
    let mut wall = Object::new_plane()
        .with_transformation(translation(1.0, 1.0, 40.0) * rotation_x(PI / 2.0))
        .with_pattern(Pattern::with_type(PatternType::ring_pattern(
            Color::new(0.2, 0.2, 0.2),
            Color::black(),
        )));

    wall.material.specular = 7.0;
    wall.material.reflective = 1.0;
    wall.material.diffuse = 1.0;
    wall.material.shininess = 1600.0;

    let mut floor =
        Object::new_plane()
            .with_color(Color::white())
            .with_pattern(Pattern::with_type(PatternType::checker_pattern(
                Color::black(),
                Color::white(),
            )));
    floor.material.color = Color::white();
    floor.material.specular = 0.0;
    floor.material.reflective = 0.1;

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
            Color::black(),
            Color::white(),
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
    left.material.specular = 0.3;
    left.material.pattern.set_transformation(
        rotation_z(2.0 * PI / 3.0) * translation(1.0, 0.0, 0.0) * scaling(2.0, 2.0, 2.0),
    );

    let mut objects = Vec::new();
    middle.move_up(1.0);
    objects.push(floor);
    objects.push(wall);
    objects.push(middle);
    objects.push(right);
    objects.push(left);

    let light = Light::new(
        Color::white(),
        scaling(2.0, 2.0, 2.0) * Point::new_point3D(10.0, 5.0, -10.0),
    );

    let world = World::new(light, objects, 5);
    let mut camera = Camera::new(1300, 1200, PI / 3.0);
    camera.with_transformation(&view_transform(
        &Point::new_point3D(0.0, 1.5, -5.0),
        &Point::new_point3D(0.0, 1.0, 0.0),
        &Vector::new_vec3D(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);
    canvas
        .convert_to_png("/home/matz/Code/Rust/rtc/output.png")
        .expect("Error");
}
