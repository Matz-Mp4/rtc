use super::ApproximateEq;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn red() -> Color {
        Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red.approx_eq_low(&other.red)
            && self.green.approx_eq_low(&other.green)
            && self.blue.approx_eq_low(&other.blue)
    }
}
impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.red - rhs.red;
        let green = self.green - rhs.green;
        let blue = self.blue - rhs.blue;

        Color { red, green, blue }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let green = self.green + rhs.green;
        let blue = self.blue + rhs.blue;

        Color { red, green, blue }
    }
}

//Scalar
impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let red = self.red * rhs;
        let green = self.green * rhs;
        let blue = self.blue * rhs;

        Color { red, green, blue }
    }
}

//Scalar
impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let red = self.red / rhs;
        let green = self.green / rhs;
        let blue = self.blue / rhs;

        Color { red, green, blue }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;

        Color { red, green, blue }
    }
}
