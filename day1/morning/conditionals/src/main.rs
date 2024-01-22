fn main() {
    let x = 10;

    // if expressions
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }


    let y = 10;
    let size: &str = if y < 20 { "small" } else { "larger" };
    println!("number size: {}", size);
}
