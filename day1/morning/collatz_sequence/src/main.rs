// function to calculate the length of the collatz sequence
// for a given initial `n`

/// Determine the length of the collatz sequence beginning at `n`
fn collatz_length(mut n: u32) -> u32 {
    // todo!("Implement this")
    let mut length: u32 = 1;
    while dbg!(n) > 1 {
        length += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
    }
    length
}

fn main() {
    // todo!("Implement this")
    println!("The length of the collatz sequence: {}", collatz_length(11))
}
