use std::io;    // We use --> import the input/output library from the standard library
                // Rust libraries are known as "crates"

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // We create a variable named guess and give the attribute of MUTable, meaning that we can change this value, by default the RUST Variables are immutable. example: let apple = 5; ----> immutable  /  let mut apple = 5; ----> mutable
                                            // the String::new() calls a method associated with the String type, which means that it creates a new, empty String.
                                            
    io::stdin() // We call the stdin() function from the io module to handle the user input
        .read_line(&mut guess)  // We call the read_line() method from the Standard input handle and then we POINT to where this String should be append with the & reference symbol that is also IMMUTABLE, hence, the mut
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}