// 7- Write a program that checks if a number is positive, negative, or zero and prints the result.
// change it to main.rs

use std::io;

fn main () {
    let mut var = String::new();
    println!("Enter a number : ");
    io::stdin().read_line(&mut var).expect("Error in reading the var");

    let num:i32 = var.trim().parse().expect("The number");
    if num == 0 {
        println!("{} is zero", num);
    } else if num > 0 {
        println!("{} is positive", num);
    } else {
        println!("{} is negative", num);
    }
}