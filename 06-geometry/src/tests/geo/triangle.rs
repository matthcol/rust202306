use crate::*;

#[test]
fn test_area_345_origin() {
    let t = Triangle(
        Point(0.0, 0.0),
        Point(0.0, 3.0),
        Point(4.0, 0.0)
    );
    assert_eq!(t.area(), 6.0);
}

#[test]
fn test_area_345_other1() {
    let t = Triangle(
        Point(1.0, 1.0),
        Point(1.0, 4.0),
        Point(5.0, 1.0)
    );
    assert_eq!(t.area(), 6.0);
}

#[test]
fn test_area_345_other2() {
    let t = Triangle(
        Point(1.0, 2.0),
        Point(1.0, 6.0),
        Point(-2.0, 2.0)
    );
    assert_eq!(t.area(), 6.0);
}