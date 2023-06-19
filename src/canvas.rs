use super::color::Color;
use std::fs::{self, File};
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

    pub fn convert_to_ppm(&self, file_name: &str) -> Result<File, std::io::Error> {
        let image = File::create(file_name);
        let mut content = String::new();
        //Header
        content.push_str("P3\n");
        content.push_str(self.width.to_string().as_str());
        content.push(' ');
        content.push_str(self.height.to_string().as_str());
        content.push_str("\n255\n");

        for i in 0..self.height {
            for j in 0..self.width {
                let mut temp = color_into_pixel(&self[i][j]);
                if j < self.width - 1 {
                    temp.push(' ');
                } else {
                    temp.push('\n');
                }
                content.push_str(temp.as_str());
            }
        }
        fs::write(file_name, content)?;
        image
    }
}

fn color_into_pixel(color: &Color) -> String {
    let mut result = String::new();
    let red = scale_color(color.red);
    let blue = scale_color(color.blue);
    let green = scale_color(color.green);
    result.push_str(red.to_string().as_str());
    result.push(' ');
    result.push_str(green.to_string().as_str());
    result.push(' ');
    result.push_str(blue.to_string().as_str());

    result
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
