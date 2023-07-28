use rtc::{transformation::scaling, Canvas, Vector};

fn main() {
    let screen = Canvas::new(600, 480);
    /* screen */
    /* .convert_to_ppm("/home/matz/Desktop/Code/Rust/rtc/test.ppm") */
    /* .expect("Error"); */

    let transform = scaling(2.0, 3.0, 4.0);
    let v = Vector::new_vec3D(-4.0, 6.0, 8.0);
    let mut expected = Vector::new_vec3D(-8.0, 18.0, 32.0);

    transform * v;

    println!("{:?}", transform);

    let inv = transform.inverse::<3>();
    println!("{:?}", inv);
    expected = Vector::new_vec3D(-4.0, 6.0, 8.0);
    let res = inv * v;
    println!("res = {:?}", res);
}
