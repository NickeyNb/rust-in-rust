// 9- Implement a basic calculator program that takes two numbers and an operator as input.
// change to main.rs

use std::io;
fn main () {
    println!("Enter first number : ");
    let mut var1 = String::new();
    io::stdin().read_line(&mut var1).expect("Error");
    let n1:i32 = var1.trim().parse().expect("Enter num1 ");

    println!("Enter second number : ");
    let mut var2 = String::new();
    io::stdin().read_line(&mut var2).expect("Error in var2");
    let n2:i32 = var2.trim().parse().expect("Enter num2 "); 

    println!("1: Addition \n2: Subtraction\n3: Multiplication\n4: Division.");
    println!("Choose thee operation : ");

    let mut var3 = String::new();
    io::stdin().read_line(&mut var3).expect("Hahah");
    let num3:i32 = var3.trim().parse().expect("Enter num3");

    match num3 {
        1=>println!("{}", n1+n2),
        2=>println!("{}", n1-n2),
        3=>println!("{}", n1*n2),
        4=>println!("{}", n1/n2),
        _=>println!("Enter valid operation ")
    }
    

}