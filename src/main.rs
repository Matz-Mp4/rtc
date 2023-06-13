use rtc::Canvas;

fn main() {
    let screen = Canvas::new(640, 480);
    screen.gen_png("/home/matz/Desktop/Code/Rust/rtc/test3.png");
    println!("Hello, world!");
}
