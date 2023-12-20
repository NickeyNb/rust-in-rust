// 5- Declare and initialize variables for a person's name, age. Print these values.

use std::io;

struct Candidate {
    c_name:String,
    c_age:u32,
}

fn main () {
    println!("Change this file to main.rs");
    let mut name: String = String::new();
    let mut age:String = String::new();
    
    // print is showing weird behaviour as instead of print it is taking input first and then printing
    println!("Enter the name : ");
    io::stdin().read_line(&mut name).expect("Enter the string");
    name = name.trim().to_string(); // converting to to_string as &str to String

    println!("Enter the age : ");
    io::stdin().read_line(&mut age).expect("Enter the string");

    let age:u32 = age.trim().parse().expect("Enter number ");
    println!("The name is {} and the age is {}", name, age);

    let cand_1: Candidate = Candidate{
        c_name:name,
        c_age:age,
    };

    println!("The name will be {} and age will be {}", cand_1.c_name, cand_1.c_age);

}
