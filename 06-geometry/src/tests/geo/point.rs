use crate::*;

#[test]
fn test_distance_same() {
    let p = Point(4.0, 5.0);
    assert_eq!(p.distance(&p), 0.0)
}

#[test]
fn test_distance_345() {
    let p1 = Point(4.0, 5.0);
    let p2 = Point(7.0, 9.0);
    assert_eq!(p1.distance(&p2), 5.0)
}