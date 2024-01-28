// 29- Write a program that reads a sentence from the user, counts the number of words, and prints the result.

use std::io;
fn main() {
    let mut str  = String::new();
    io::stdin().read_line(&mut str).expect("Error in reading");

    let mut i: usize=0;
    while i < str.len() {
        i+=1;
    }

    println!("{}", i-1);
}