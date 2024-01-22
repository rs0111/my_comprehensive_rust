fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        // todo!("Implement this")

        // The first and second Fibonacci numbers are both 1
        1
    } else {
        // The recursive case
        // todo!("Implement this")

        // sum of the (n-1)'th and (n-2)'th Fibonacci numbers
        fib(n - 1) + fib(n - 2)
    }
}


fn main() {
    let n = 20;
    println!("fib(n) = {}", fib(n));
}
