use std::f64::consts::PI;

use rtc::{
    animator::Frame,
    color::Color,
    transformation::{rotation_x, rotation_y, rotation_z, scaling, translation, view_transform},
    Animator, Camera, Light, Object, Pattern, PatternType, Point, Vector, World,
};

fn main() {
    let mut wall = Object::new_plane();
    wall.material.color = Color::new(0.0, 0.0, 0.0);
    wall.material.specular = 1.0;
    wall.material.reflective = 1.0;
    wall.material.diffuse = 1.0;
    wall.set_transformation(translation(-2.0, 2.0, 2.0) * rotation_x(PI / 2.0));
    /*wall.material
    .set_pattern(Pattern::with_type(PatternType::checker_pattern(
        Color::white(),
        Color::black(),
    )));*/

    let mut floor = Object::new_plane();
    floor.material.color = Color::white();
    floor.material.specular = 0.0;
    floor.material.reflective = 1.0;
    floor
        .material
        .set_pattern(Pattern::with_type(PatternType::checker_pattern(
            Color::black(),
            Color::white(),
        )));

    let mut middle = Object::new_sphere();
    middle.set_transformation(translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::black();
    middle.material.reflective = 1.0;
    middle.material.shininess = 1600.0;
    middle.material.specular = 5.0;
    middle.material.diffuse = 0.1;

    let mut right = Object::new_sphere();
    right.set_transformation(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right
        .material
        .set_pattern(Pattern::with_type(PatternType::ring_pattern(
            Color::red(),
            Color::blue(),
        )));
    right
        .material
        .pattern
        .set_transformation(scaling(0.002, 0.002, 1.0) * rotation_y(-PI / 4.0));
    right.material.color = Color::red();

    let mut left = Object::new_sphere();
    left.set_transformation(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 1.0;
    left.material.specular = 0.3;
    left.material
        .set_pattern(Pattern::with_type(PatternType::gradient_pattern(
            Color::blue(),
            Color::red(),
        )));
    left.material.pattern.set_transformation(rotation_z(2.0*PI / 3.0) * translation(1.0,0.0,0.0) * scaling(2.0,2.0,2.0));

    let mut objects = Vec::new();

    objects.push(floor);
    objects.push(wall);
    objects.push(middle);
    objects.push(right);
    objects.push(left);

    let light = Light::new(Color::white(), Point::new_point3D(-10.0, 10.0, -10.0));

    let world = World::new(light, objects, 5);
    let mut camera = Camera::new(1200, 1200, PI / 3.0);
    camera.with_transformation(&view_transform(
        &Point::new_point3D(0.0, 1.5, -5.0),
        &Point::new_point3D(0.0, 1.0, 0.0),
        &Vector::new_vec3D(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);
    canvas
        .convert_to_ppm("/home/matz/Code/Rust/rtc/output.ppm")
        /* .convert_to_png("/home/matz/Desktop/Code/Rust/rtc/output.png") */
        .expect("Error");
}
