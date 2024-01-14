// 10- Create a function that calculates the area of a rectangle given its length and width.
// change it to main.rs

use std::io;
fn main () {
    let mut var1 = String::new();
    let mut var2 = String::new();
    
    println!("Enter the length : ");
    io::stdin().read_line(&mut var1).expect("Error in reading var1");
    let len:i32 = var1.trim().parse().expect("Enter the length ");

    println!("Enter the breadth : ");
    io::stdin().read_line(&mut var2).expect("Error in reading var2");
    let bre:i32 = var2.trim().parse().expect("Enter the breadth ");

    println!("The area of rectangle is : {}",area_of_rectangle(len, bre));
}

fn area_of_rectangle(length:i32, breadth:i32)->i32 {
    length*breadth
}