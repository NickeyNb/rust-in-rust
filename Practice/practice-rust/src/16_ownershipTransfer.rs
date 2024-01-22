// 16- Write a program that demonstrates the ownership transfer between variables.

fn takes_ownership(s:String) {
    println!("Takes ownership : {}", s);
}

fn move_variable(n:u32) {
    println!("Move variable : {}", n);

}

fn passed_reference(s2:&String) {
    println!("Passed reference : {}", s2);
}


fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("{}", s1); // we can't do this as s1 is passed to takes_ownership funciton

    let n1:u32 = 5;
    move_variable(n1);
    println!("{}", n1); // it is moved not transferred

    let s2 = String::from("world");
    passed_reference(&s2);
    println!("{}", s2); // passed as reference
    
}