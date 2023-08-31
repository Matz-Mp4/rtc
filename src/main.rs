use std::f64::consts::PI;

use rtc::{
    animator::Frame,
    color::Color,
    transformation::{rotation_x, rotation_y, rotation_z, scaling, translation, view_transform},
    Animator, Camera, Light, Object, Pattern, PatternType, Point, Vector, World,
};

fn main() {
    let mut floor = Object::new_plane();
    floor.material.color = Color::new(0.3, 0.9, 0.9);
    floor.material.specular = 0.0;
    floor
        .material
        .set_pattern(Pattern::with_type(PatternType::ring_pattern(
            Color::white(),
            Color::black(),
        )));

    let mut middle = Object::new_sphere();
    middle.set_transformation(translation(-0.5, 1.0, 0.5) * rotation_z(PI / 2.0));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle
        .material
        .set_pattern(Pattern::with_type(PatternType::checker_pattern(
            Color::black(),
            Color::white(),
        )));

    let mut right = Object::new_sphere();
    right.set_transformation(translation(0.2, 0.5, 0.5) * scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Object::new_sphere();
    left.set_transformation(translation(-1.2, 0.5, 0.5) * scaling(0.5, 0.5, 0.5));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 1.0;
    left.material.specular = 0.3;

    let mut objects = Vec::new();

    objects.push(floor);
    objects.push(middle);
    objects.push(right);
    objects.push(left);

    let light = Light::new(Color::white(), Point::new_point3D(-10.0, 10.0, -10.0));

    let world = World::new(light, objects);
    let mut camera = Camera::new(1680, 900, PI / 3.0);
    camera.with_transformation(&view_transform(
        &Point::new_point3D(0.0, 1.5, -5.0),
        &Point::new_point3D(0.0, 1.0, 0.0),
        &Vector::new_vec3D(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);
    canvas
        /* .convert_to_ppm("./home/matz/Code/Rust/rtc/output.ppm") */
        .convert_to_png("/home/matz/Code/Rust/rtc/output.png")
        .expect("Error");
}
