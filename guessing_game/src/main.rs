use std::io;
use std::cmp::Ordering;
use rand::Rng;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main_will_crash() {
    println!( "Guess the number!" );
    println!( "Type your guess: " );
    
    // 'let' is used to declare a new variable
    
    // 'mut' overwrites the default 
    // immutability of variables
    
    // String::new() returns a new (empty) instance of
    // type String; the :: indicates that 'new'
    // is an associated function of the String 
    // type (analogous to static method
    let mut guess = String::new();

    // Rng is a trait that defines methods that random number
    // generators implement (more on this later)
    // rand::thread_rng() is a function that returns a 
    // random number generator. 
    // .gen_range() is a method that actually returns the 
    // random number by defining the range of possible values
    let answer = rand::thread_rng().gen_range(1, 101);
    // io::stdin() returns an instance of std::io::Stdin
    // .read_line( &mut guess ) calls the read_line method
    // and saves it to the _mutable_ _reference_ specified
    // by &mut guess

    // read_line also returns a io::Result object;
    // the Result types are enumerations (aka enums); 
    // enums have a fixed set of possible values (aka variants).
    // for io::Result, the variants are Ok and Err;
    // Ok indicates the operation succeeded, and Err that it 
    // failed (and the Err will have some helpful info).

    // the io::Result type has a .expect() method; 
    // if the io::Result variant is Err, the program will
    // crash and display the message passed to .expect().
    io::stdin().read_line( &mut guess ).expect( "Failed to read line" );
    
    // {} is a placeholder to print a variable. 
    // you can have multiple {}s.
    println!( "You guessed {}", guess );


    // Ordering is another enum, whose variants are Less,
    // Greater, and Equal.

    // the 'match' expression is used to decide
    // what to do based on the result of cmp()

    // 'matchs' are made of up arms; each arm is a pattern
    // and the associated code that should be run if the
    // value given to the beginning of the match expression
    // fits the arm's patter.
    
    // since we're comparing a String to a number, the
    // program will crash
    //match guess.cmp( &answer ) {
    //    Ordering::Less => println!( "Too small" ),
    //    Ordering::Greater => println!( "Too big" ),
    //    Ordering::Equal => println!( "Got it!" ),
    //}
}

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
