fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();

    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);

    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);

    // NOTE valid because `&str` = `&'static str`
    // &str does not own the string slice
    println!("{:?}", greeting);

    // panic with 🪐
    //println!("{:?}", &sentence[12..13]);
}
