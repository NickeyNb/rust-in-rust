// 12- Implement a function that checks if a number is prime.

use std::io;
fn main() {
    let mut var = String::new();
    println!("Enter the number greater than 1 ");
    
    io::stdin().read_line(&mut var).expect("Error in reading ");
    let num:i32 = var.trim().parse().expect("Enter the number : ");
 
    if check_prime(num) == 0 {
        println!("The number is prime ");
    } else {
        println!("The number is not prime ");
    }
}

fn check_prime(num:i32)->i32 {

    let mut i=2;
    while i<num {
        if num%i == 0 {
            return -1
        } 
        i += 1;
    }
    return 0
}