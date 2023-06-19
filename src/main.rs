use rtc::Canvas;

fn main() {
    //TODO: Refactor vector and point to default trait 
    let screen = Canvas::new(600, 480);
    screen
        .convert_to_ppm("/home/matz/Desktop/Code/Rust/rtc/test.ppm")
        .expect("Error");
    println!("Hello, world!");
}
