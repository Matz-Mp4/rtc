use rtc::{Canvas, Matrix};

fn main() {
    let matx = Matrix::from([[-3, 6, 12], [-1, 3, 5], [-1, 9, 25]]);
    matx.det();

    /*
    let screen = Canvas::new(600, 480);
    screen
        .convert_to_ppm("/home/matz/Desktop/Code/Rust/rtc/test.ppm")
        .expect("Error");*/
}
