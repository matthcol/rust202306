use std::f64::consts::PI;

trait Measurable2d {
    fn area(&self) -> f64;

    fn perimeter(&self) -> f64; 
}

#[derive(Debug)]
struct Point(f64, f64);

#[derive(Debug)]
struct Circle {
    #[allow(dead_code)]
    center: Point,
    radius: f64
}

impl Measurable2d for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn play_with_measurable() {
    let c = Circle{
        center: Point(0.0, 0.0),
        radius: 3.0
    };
    let a = c.area();
    let p = c.perimeter();
    println!("{c:?}: perimeter = {a}, area = {p}")
}



fn main() {
    play_with_measurable()
}
