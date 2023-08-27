use std::f64::consts::PI;

use rtc::{
    color::Color,
    transformation::{rotation_x, rotation_y, scaling, translation, view_transform},
    Animator, Camera, Light, Object, Point, Vector, World,
};

fn main() {
    let animator = Animator::new(25 * 5);
    animator.animate(|frame| {
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
        let righ_translation_scale = frame.linear_scale().with_range(0.2, 3.0);
        right.set_transformation(
            translation(
                1.5,
                righ_translation_scale.scale(frame.current_as_float()),
                -0.5,
            ) * scaling(0.5, 0.5, 0.5),
        );
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
        objects.push(left_wall);
        objects.push(right_wall);
        objects.push(middle);
        objects.push(right);
        objects.push(left);

        let light_rotation_scale = frame.linear_scale().with_range(0.0, PI * 2.0);
        let light_transformation_matrix =
            rotation_y(light_rotation_scale.scale(frame.current_as_float()));

        let light = Light::new(
            Color::white(),
            light_transformation_matrix * Point::new_point3D(-10.0, 10.0, -10.0),
        );

        let world = World::new(light, objects);
        let mut camera = Camera::new(1680, 1680, PI / 3.0);
        camera.with_transformation(&view_transform(
            &Point::new_point3D(0.0, 1.5, -5.0),
            &Point::new_point3D(0.0, 1.0, 0.0),
            &Vector::new_vec3D(0.0, 1.0, 0.0),
        ));

        let filename = frame.file_name("/home/matz/Code/Rust/rtc/videos/", "output", "png");
        println!("filename = {}", filename);

        let canvas = camera.render(&world);
        canvas
            /* .convert_to_ppm("./home/matz/Code/Rust/rtc/output.ppm") */
            .convert_to_png(filename.as_str())
            .expect("Error");
    });
}
