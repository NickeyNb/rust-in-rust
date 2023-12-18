// Taking user-input as String as well as number(u32)

use std::io;    // importing standard input/output module

fn main() {

    let mut user_input = String::new();
    println!("Enter the text");

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    // read_line requires a mutable reference 
    // reads a line from the input and stores it in the buffer user_input variable.

    // println!("Your input-string is {}", user_input);   // it includes \n in the i/p.
    user_input = user_input.trim().to_string(); // trimming that \n and converting &str to String
    println!("Your input-string is {}", user_input); // without \n

    // Here, we use parse to convert from a string to a number.
    let parsed_number:i32 = user_input.trim().parse().expect("Enter a number"); 
    println!("Parsed number is {}", parsed_number);

    
}