use super::{point::Point, mesurable2d::Measurable2d};

#[derive(Debug)]
pub struct Triangle(
    pub Point,
    pub Point,
    pub Point,
);

impl Measurable2d for Triangle {
    fn area(&self) -> f64 {
        let a = self.0.distance(&self.1);
        let b = self.1.distance(&self.2);
        let c = self.2.distance(&self.0);
        let p = (a+b+c) / 2.0;
        (p*(p - a)*(p - b)*(p-c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.0.distance(&self.1)
        + self.1.distance(&self.2)
        + self.2.distance(&self.0)
    }    
}