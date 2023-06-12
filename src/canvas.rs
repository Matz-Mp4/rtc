use super::color::Color;
use image::ColorType;
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
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn write_pixel(&mut self, pixel: Color) {
        self.pixels.push(pixel);
    }

    pub fn gen_png(&self, path: &str) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self[y as usize][x as usize];
            let r = scale_color(color.red);
            let g = scale_color(color.green);
            let b = scale_color(color.blue);

            *pixel = image::Rgb([r, g, b]);
        }

        img.save(path)
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
