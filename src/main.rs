use rtc::Canvas;

fn main() {
    let screen = Canvas::new(600, 480);
    screen
        .convernt_to_ppm("/home/matz/Code/Rust/rtc/test.ppm")
        .expect("Error");
    println!("Hello, world!");
}
