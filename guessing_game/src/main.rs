use std::io;
use std::cmp::Ordering;
use rand::Rng;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    let answer = rand::thread_rng().gen_range(1, 101);

    println!( "Guess the number!" );

    // loop until the program crashes
    // (e.g. invalid input, but we've got a 
    // fix for that) or we hit the break statement
    loop {
        println!( "Type your guess: " );

        let mut guess = String::new();
        io::stdin().read_line( &mut guess ).expect( "Failed to read line" );

        // normally we wouldn't be allowed to have
        // 2 variables with the same name, but Rust
        // allows us to "shadow" the previous value
        // of 'guess' with the new one.

        // since we know guess will be a String,
        // we first call .trim() to trim the whitespace
        // at the start and end of the string.AsMut
        
        // the 'parse' method on strings parses a string into
        // some kind of number (here we are getting a u32 back).
        // the .expect(...) method is used in case the casting of
        // (String -> u32) fails.

        // was originally let guess: u32 = guess.trim().parse().expect("Please type an integer!" );
        // but we changed it to allow the program
        // to ignore invalid inputs
        // note that this is a match expression!
        // match expressions are the basic building blocks
        // or error handling.
        let guess: u32 = match guess.trim().parse() {
            Ok( some_num ) => some_num, // if the parse function was successful, => ...
            Err( _ ) => continue, // _ is a catch all
        }; 
        

        println!( "You guessed {}", guess );

        match guess.cmp( &answer ) {
            Ordering::Less => println!( "Too small" ),
            Ordering::Greater => println!( "Too big" ),
            Ordering::Equal => {
                println!( "Got it!" );
                break;
            }
        }
    } 
}
