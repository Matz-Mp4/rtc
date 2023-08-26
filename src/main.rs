use std::f64::consts::PI;

use rtc::{
    color::Color,
    transformation::{rotation_x, rotation_y, scaling, translation, view_transform},
    Camera, Canvas, Light, Object, Point, Ray, Vector, World,
};

fn main() {
    let mut floor = Object::new_sphere();
    floor.set_transformation(scaling(10.0, 0.01, 10.0));
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Object::new_sphere();
    left_wall.set_transformation(
        translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
    );
    left_wall.material = floor.material;

    let mut right_wall = Object::new_sphere();

    right_wall.set_transformation(
        translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
    );
    right_wall.material = floor.material;

    let mut middle = Object::new_sphere();
    middle.set_transformation(translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Object::new_sphere();
    right.set_transformation(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Object::new_sphere();
    left.set_transformation(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut objects = Vec::new();

    objects.push(floor);
    /* objects.push(left_wall); */
    /* objects.push(right_wall); */
    objects.push(middle);
    objects.push(right);
    objects.push(left);

    let light = Light::new(Color::white(), Point::new_point3D(-10.0, 10.0, -10.0));

    let world = World::new(light, objects);
    let mut camera = Camera::new(800, 800, PI / 3.0);
    camera.with_transformation(&view_transform(
        &Point::new_point3D(0.0, 1.5, -5.0),
        &Point::new_point3D(0.0, 1.0, 0.0),
        &Vector::new_vec3D(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);
    canvas
        .convert_to_ppm("/home/matz/Code/Rust/rtc/test2.ppm")
        .expect("Error");
}
