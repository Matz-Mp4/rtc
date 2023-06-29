use rtc::Canvas;

fn main() {
    let screen = Canvas::new(600, 480);
    screen
        .convert_to_ppm("/home/matz/Desktop/Code/Rust/rtc/test.ppm")
        .expect("Error");
}
