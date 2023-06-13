use super::color::Color;
use std::fs::File;
use std::ops::{Index, IndexMut};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::red(); width * height],
        }
    }

    pub fn write_pixel(&mut self, pixel: Color) {
        self.pixels.push(pixel);
    }

    pub fn gen_png(&self, file_name: &str) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);
        let _output = File::create(file_name);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self[y as usize][x as usize];
            let r = scale_color(color.red);
            let g = scale_color(color.green);
            let b = scale_color(color.blue);

            *pixel = image::Rgb([r, g, b]);
        }

        img.save(file_name)
    }
}

fn scale_color(component: f64) -> u8 {
    let component = if component < 0.0 {
        0.0
    } else if component > 1.0 {
        1.0
    } else {
        component
    };

    (component * 255.0) as u8
}

impl Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.width;

        &self.pixels[start..start + self.width]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.width;

        &mut self.pixels[start..start + self.width]
    }
}
