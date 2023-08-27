use std::thread;
use std::time::Duration;

pub struct Animator {
    frame_count: usize,
}

pub struct LinearScale {
    domain: (f64, f64),
    range: (f64, f64),
}

impl LinearScale {
    pub fn new() -> Self {
        Self {
            domain: (0.0, 100.0),
            range: (0.0, 1.0),
        }
    }

    pub fn with_domain(mut self, start: f64, end: f64) -> Self {
        self.domain = (start, end);
        self
    }

    pub fn with_range(mut self, start: f64, end: f64) -> Self {
        self.range = (start, end);
        self
    }

    pub fn scale(&self, input: f64) -> f64 {
        let clamped_input = input.clamp(self.domain.0, self.domain.1);
        let offsetted_input = clamped_input - self.domain.0;
        let normalized_input = offsetted_input / (self.domain.1 - self.domain.0);
        let offsetted_output = normalized_input * (self.range.1 - self.range.0);
        let output = self.range.0 + offsetted_output;
        output.clamp(self.range.0, self.range.1)
    }
}

pub struct Frame {
    current: usize,
    count: usize,
}

impl Frame {
    pub fn new(current: usize, count: usize) -> Self {
        Self { current, count }
    }

    pub fn file_name(&self, path: &str, name: &str, suffix: &str) -> String {
        format!("{}{}{:05}.{}", path, name, self.current, suffix)
    }
    pub fn linear_scale(&self) -> LinearScale {
        LinearScale::new().with_domain(0.0, self.count as f64)
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn current_as_float(&self) -> f64 {
        self.current as f64
    }
}

impl Animator {
    pub fn new(frame_count: usize) -> Self {
        Self { frame_count }
    }

    pub fn animate(&self, animete: fn(frame: Frame)) {
        for current_frame in 0..self.frame_count {
            let frame_count = self.frame_count;
            animete(Frame::new(current_frame, frame_count));
        }
    }
}
