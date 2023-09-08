use std::ops::Mul;

use crate::transformation::*;
use crate::Matrix;

pub trait Motion: Sized {
    fn move_front(&mut self, value: f64) -> Self;
    fn move_back(&mut self, value: f64) -> Self;
    fn move_left(&mut self, value: f64) -> Self;
    fn move_right(&mut self, value: f64) -> Self;
    fn move_up(&mut self, value: f64) -> Self;
    fn move_down(&mut self, value: f64) -> Self;
}
