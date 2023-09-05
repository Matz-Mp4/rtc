use crate::{Canvas, Matrix, Point, Ray, Vector, World};
use colored::{self, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub transform: Matrix<f64, 4, 4>,
    pub inverse_transform: Matrix<f64, 4, 4>,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let (pixel_size, half_width, half_height) = Camera::compute_pixel_size(hsize, vsize, fov);
        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::iden(),
            inverse_transform: Matrix::iden(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let pb = ProgressBar::new(image.pixels().len() as u64);

        println!("{}", "RayTracing...".italic().bold());
        (0..self.hsize)
            .flat_map(move |px| (0..self.vsize).map(move |py| (px, py)))
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(px, py)| {
                let ray = self.ray_for_pixel(px, py);
                pb.inc(1);
                let color = world.color_at(&ray, world.reflection_limit);
                (px, py, color)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(px, py, color)| {
                image[py][px] = color;
            });
        image
    }
    pub fn render2(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let pb = ProgressBar::new(image.pixels().len() as u64);
        println!("{}", "RayTracing...".italic().bold());

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, world.reflection_limit);
                image[y][x] = color;

                pb.inc(1);
            }
        }

        image
    }

    fn compute_pixel_size(hsize: usize, vsize: usize, fov: f64) -> (f64, f64, f64) {
        let half_view = f64::tan(fov / 2.0);
        let aspect = hsize as f64 / vsize as f64;
        let mut half_width = 0.0;
        let mut half_height = 0.0;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        (pixel_size, half_width, half_height)
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = self.inverse_transform * Point::new_point3D(world_x, world_y, -1.0);
        let origin = self.inverse_transform * Point::new_point3D(0.0, 0.0, 0.0);
        let direction = Vector::normalize(pixel - origin);

        Ray::new(origin, direction)
    }

    pub fn with_transformation(&mut self, transform: &Matrix<f64, 4, 4>) {
        self.transform = *transform;
        self.inverse_transform = transform.inverse();
    }
}
