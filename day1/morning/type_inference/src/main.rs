fn takes_u32(x: u32) {
    println!("u32: {x}");
}


fn takes_i8(y: i8) {
    println!("i8: {y}");
}


fn main() {
    let x = 10;
    let y = 20;

    // type inference
    takes_u32(x);
    takes_i8(y);

    // error with type
    //takes_u32(y);
}
