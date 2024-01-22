fn main() {
    let mut i = 0;

    // The `loop` statement just loops forever,
    // until a `break`
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
