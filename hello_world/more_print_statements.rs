fn main() {
    /*
    * Cooler things we can do with print statements in Rust.
    */
    println!("{} days until tomorrow.", 1);
    
    // specify which variable to print by index
    println!("{0} loves {1}, and {1}, loves {2}", "Bob", "Alice", "Alex");
    
    // to escape the {}, nest them inside an encompassing {} pair!
    println!("The {{}} notation also works with kwargs:");
    
    // println! also supports kwargs!
    println!("{subject} {verb} {object}", 
        object="the lazy dog", 
        subject="the quick brown fox", 
        verb="jumped over"
    );

    /* 
    * Special formatting can be specified after a `:`.
    * for instance, we can convert a number to binary and
    * print the corresponding number
    */ 
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);
 

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    let pi = 3.141592;

    // formatting to a specific number of decimal places:
    // see: https://doc.rust-lang.org/std/fmt/
    println!("pi={}", format!("{:.3}", pi));

    // or alternatively,

    println!("pi={}", format!("{:.*}", 3, pi));
}