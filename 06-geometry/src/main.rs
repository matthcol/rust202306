use geo::point::Point;
use geo::circle::Circle;
use geo::mesurable2d::{sum_area, sum_area2, Measurable2d};
use geo::triangle::Triangle;

pub mod geo;

#[cfg(test)]
mod tests;




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
fn sum_area_circle<'a, I>(mut iterator: I) -> f64
where I: Iterator<Item=&'a Circle>
{
    let mut sum = 0.0;
    while let Some(e) = iterator.next() {
        sum += e.area();
    }
    sum
}

fn sum_area_circle2<'a, I>(iterator: I) -> f64
where I: Iterator<Item=&'a Circle>
{
    iterator.map(Circle::area).sum()
}


fn play_with_sum_area_circle() {
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
    let s = sum_area_circle(circles.iter());
    println!("Total area: {s}"); 

    let s = sum_area_circle2(circles.iter());
    println!("Total area: {s}");

    let s = sum_area_circle2(circles[..2].iter());
    println!("Total area: {s}");

    let s = sum_area(circles[..2].iter());
    println!("Total area: {s}");

    let s = sum_area2(circles[..2].iter());
    println!("Total area: {s}");
}

fn play_with_sum_area_triangle(){
    let triangles = vec![
        Triangle(
            Point(1.0, 2.0),
            Point(1.0, 6.0),
            Point(-2.0, 6.0)
        ),
        Triangle(
            Point(1.0, 2.0),
            Point(1.0, 5.0),
            Point(-3.0, 5.0)
        ),
        Triangle(
            Point(1.0, 2.0),
            Point(1.0, -1.0),
            Point(5.0, -1.0)
        )
    ];

    let s = sum_area(triangles.iter());
    println!("Total area: {s}");
}

fn main() {
    play_with_measurable();
    play_with_sum_area_circle();
    play_with_sum_area_triangle();
}
