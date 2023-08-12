use rtc::{
    color::Color, transformation::scaling, Canvas, Light, Material, Object, Point, Ray, Shape,
    Vector,
};

fn main() {
    /* let screen = Canvas::new(600, 480); */
    /* screen */
    /* .convert_to_ppm("/home/matz/Desktop/Code/Rust/rtc/test.ppm") */
    /* .expect("Error"); */

    let ray_origin = Point::new_point3D(0.0, 0.0, -5.0);
    let wall_z = 8.0;
    let wall_size = 10.0;
    let canvas_pixels = 800;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);

    let mut object = Object::new_sphere();
    object.set_color(Color::new(1.0, 0.2, 1.0));

    let light_positon = Point::new_point3D(-10.0, 10.0, -10.0);
    let light_color = Color::white();
    let light = Light::new(light_color, light_positon);

    for y in 0..canvas_pixels - 1 {
        //compute the world y coordinate
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels - 1 {
            //compute the world x coordinate
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new_point3D(world_x as f64, world_y as f64, wall_z as f64);
            let ray = Ray::new(ray_origin, Vector::normalize(position - ray_origin));
            let hit = object.intersects(&ray);

            if let Some((t1, t2)) = hit {
                let t = {
                    if t1 < t2 {
                        t1
                    } else {
                        t2
                    }
                };
                let point = ray.position(t);
                let eye = -ray.direction;
                let normal = object.normal_at(&point);
                let color = object.material.lightning(&light, &point, &eye, &normal);
                canvas[x as usize][y as usize] = color;
            }
        }
    }

    canvas
        .convert_to_ppm("/home/matz/Code/Rust/rtc/test.ppm")
        .expect("Error");
}
