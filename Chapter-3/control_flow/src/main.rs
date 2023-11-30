// 3.5 Control flow
//  Blocks of code associated with the conditions in if expressions are sometimes called arms
// fn main() {
//     let age:i32=21;
//     if age > 18 {   // if arm
//         println!("Age = {age}, you can go to party");
//     } else {    // else arm
//         println!("Age = {age}, you can't go to party");
//     }
//     another_fn();
// }
// Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition. If we want the if code block to run only when a number is not equal to 0

// Using if in a let statement
// fn another_fn() {
//     let val = true;
//     let num = if val {5} else {10};

//     println!("The number is {num}") // 5
// }

// ================================

// loop, while and for
//  Repeating Code with loop
fn main() {
    loop {
        let hello="hello";  // infinite-time we have to specify condition for break;
        print!("{hello}");
        break;
        
    }
    
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    for element in a {
        println!("the value is: {element}");
    }
}


