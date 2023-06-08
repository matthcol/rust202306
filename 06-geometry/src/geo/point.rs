#[derive(Debug)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: &Self) -> f64 {
        (self.0 - other.0).hypot(self.1 - other.1)
    }
}