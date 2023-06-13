use rtc::canvas::Canvas;

fn main() {
    let screen = Canvas::new(640, 480);
    screen.gen_png("/home/matz/Code/Rust/rtc/test2.png");
    println!("Hello, world!");
}
