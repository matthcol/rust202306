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

// calculer la surface totale d'objets mesurables
// NB: ref lifetime
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn sum_area<'a, I>(mut iterator: I) -> f64
where I: Iterator<Item=&'a Circle>
{
    let mut sum = 0.0;
    while let Some(e) = iterator.next() {
        sum += e.area();
    }
    sum
}

fn sum_area2<'a, I>(iterator: I) -> f64
where I: Iterator<Item=&'a Circle>
{
    iterator.map(Circle::area).sum()
}

fn play_with_sum_area() {
    let circles = vec![
        Circle{
            center: Point(0.0, 0.0),
            radius: 3.0
        },
        Circle{
            center: Point(0.0, 0.0),
            radius: 6.0
        },
        Circle{
            center: Point(0.0, 0.0),
            radius: 9.0
        }   
    ];
    let s = sum_area(circles.iter());
    println!("Total area: {s}"); 

    let s = sum_area2(circles.iter());
    println!("Total area: {s}");

    let s = sum_area2(circles[..2].iter());
    println!("Total area: {s}");
}

fn main() {
    play_with_measurable();
    play_with_sum_area()
}
