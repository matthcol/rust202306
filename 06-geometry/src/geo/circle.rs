use std::f64::consts::PI;

use super::{point::Point, mesurable2d::Measurable2d};

#[derive(Debug)]
pub struct Circle {
    #[allow(dead_code)]
    pub center: Point,
    pub radius: f64
}

impl Measurable2d for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}
