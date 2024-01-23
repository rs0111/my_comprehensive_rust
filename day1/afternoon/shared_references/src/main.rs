fn main() {
    let a = 'A';
    let b = 'B';

    // borrowing
    let mut r: &char = &a;

    println!("r: {}", *r);

    // borrowing
    r = &b;

    println!("r: {}", *r);
}


// Rust will statically forbid dangling references
/* 
fn x_axis(x: i32) -> &(i32, i32) {
    let point = (x, 0);

    return &point;
}
*/
