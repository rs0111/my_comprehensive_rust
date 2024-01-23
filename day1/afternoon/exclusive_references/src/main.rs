fn main() {
    let mut point = (1, 2);

    // exclusive references, also known as mutable references
    let x_coord = &mut point.0;

    *x_coord = 20;
    println!("point: {point:?}");
}
