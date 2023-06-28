use rtc::{Canvas, Matrix};

fn main() {
    let matx = Matrix::from([[1, 2, 6], [-5, 8, -4], [2, 6, 4]]);
    let expected = 56;
    matx.cofactor::<2>(0, 0);

    /*
    let screen = Canvas::new(600, 480);
    screen
        .convert_to_ppm("/home/matz/Desktop/Code/Rust/rtc/test.ppm")
        .expect("Error");*/
}
