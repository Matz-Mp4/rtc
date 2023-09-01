use super::color::Color;
use colored::Colorize;
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

    pub fn set_pixels(&mut self, pixels: Vec<Color>)  {
        self.pixels = pixels;
    }

    pub fn pixels(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }
    pub fn write_pixel(&mut self, pixel: Color) {
        self.pixels.push(pixel);
    }

    pub fn convert_to_png(&self, file_name: &str) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);
        let _output = File::create(file_name);

        println!("{}{}" ,"Writing ".green().bold() , file_name);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self[y as usize][x as usize];
            let r = scale_color(color.red);
            let g = scale_color(color.green);
            let b = scale_color(color.blue);

            *pixel = image::Rgb([r, g, b]);
            
        }

        img.save(file_name)
    }

    pub fn convert_to_ppm(&self, file_name: &str) -> Result<File, std::io::Error> {
        println!("Writing {}", file_name);
        let image = File::create(file_name);

        let total_size = self.width * self.height;
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
