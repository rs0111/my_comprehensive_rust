fn factorial(n: u32) -> u32 {
    let mut product = 1;

    // `dbg(expression)` logs the value of the expression
    // and return it
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}


fn fizzbuzz(n: u32) -> u32 {

    // if executed, it will panic
    todo!()
}


fn main() {
    let n = 4;

    println!("{n}! = {}", factorial(n));
}
