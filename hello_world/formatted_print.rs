fn main() {
    let age = 23;
    let age_string = format!("{}", age);
    let message = format!("Hello, I am Samy and I'm {} years old.", age_string);
    print!("{}", message);
    print!(" And this is on the same line.");
    println!("\nManually writing this on the next line, but the next line should be on a new line...");
    println!("TADA! This is automagically printed on a new line!");
}