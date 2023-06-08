use crate::*;
use assert_float_eq::*;

#[test]
fn test_area() {
    let c = Circle{
        center: Point(0.0, 0.0),
        radius: 4.0
    };
    assert_float_absolute_eq!(
        c.area(), 50.26548245743669, 1E-10
    )

}

#[test]
fn test_perimeter() {
    let c = Circle{
        center: Point(0.0, 0.0),
        radius: 4.0
    };
    assert_float_absolute_eq!(
        c.perimeter(), 25.132741228718345, 1E-10
    )
}