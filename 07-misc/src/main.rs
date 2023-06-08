

// exists in generic mode in rust api: std::mem::swap
fn swap(x: &mut i32, y: &mut i32){
    let tmp = *x;
    *x = *y;
    *y = tmp;
}

fn main() {
    let mut x = 4;
    let mut y = 5;
    swap(&mut x, &mut y);
    assert_eq!(x, 5);
    assert_eq!(y, 4);
}
