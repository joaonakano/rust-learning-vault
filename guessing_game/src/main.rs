use std::io;    // We use --> import the input/output library from the standard library
                // Rust libraries are known as "crates"

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // We create a variable named guess and give the attribute of MUTable, meaning that we can change this value, by default the RUST Variables are immutable. example: let apple = 5; ----> immutable  /  let mut apple = 5; ----> mutable
                                            // the String (type) ::new() means that we create a new instance fo the String type
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}