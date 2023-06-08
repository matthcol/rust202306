pub trait Measurable2d {
    fn area(&self) -> f64;

    fn perimeter(&self) -> f64; 
}

pub fn sum_area<'a, T>(iterator: impl Iterator<Item=&'a T>) -> f64
where 
    T: Measurable2d + 'a,
{
    iterator.map(Measurable2d::area).sum()
}


pub fn sum_area2<'a, I, T>(iterator: I) -> f64
where 
    I: Iterator<Item=&'a T>,
    T: Measurable2d + 'a,
{
    iterator.map(Measurable2d::area).sum()
}
