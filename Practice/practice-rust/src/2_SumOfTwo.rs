// You want to input two integers, and output their sum.
use std::io;
fn main() {
    println!("Welcome to practice world !!");
    let mut num1: String = String::new();
    let mut num2: String = String::new();

    println!("Enter first number : ");
    io::stdin().read_line(&mut num1).expect("Enter a number");

    println!("Enter second number : ");
    io::stdin().read_line(&mut num2).expect("Enter a number");

    let number1:u32 = num1.trim().parse().expect("Please enter a number");
    let number2:u32 = num2.trim().parse().expect("Please enter a number ");

    let total: u32 = number1 + number2;
    println!("Sum of {} and {} is : {}", number1, number2, total);
    


}