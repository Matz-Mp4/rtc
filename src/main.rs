use rtc::{
    color::Color,
    transformation::{scaling, translation},
    Canvas, Object, Point, Ray, Shape, Vector,
};

fn main() {
    let p = Point::new_point3D(-3.0, 4.0, 5.0);
    let mut transform = translation(5.0, -3.0, 2.0);
    /* let mut expected = Point::new_point3D(2.0, 1.0, 7.0); */
    /* let mut res = transform * p; */

    /* assert_eq!(expected, res); */

    /* let inv = transform.inverse::<3>(); */
    /* let inv = transform.pog_inverse(); */
    println!("transformation: {:?}", transform);
    transform.pog_inverse();
    println!("Inverse: {:?}", transform);
    /* res = inv * p; */

    /* expected = Point::new_point3D(-8.0, 7.0, 3.0); */
    /* assert_eq!(expected, res); */
}
