// Chatper-2
use std::io;
// To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // println! is a macro that prints a string to the screen:

    let mut guess = String::new();
    // The equal sign (=) tells Rust we want to bind something to the variable(guess) now.
    // String::new, a function that returns a new instance of a String.
    // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string.
    // In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // If we hadn’t imported the io library with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin.
    
    println!("You guessed: {guess}");
}