// fn main() {
//     // -----Ownership rules-----
//     // 1. Each value in Rust has a variable that's called its owner.
//     // 2. There can only be one owner at a time.
//     // 3. When the owner goes out of scope, the value will be dropped

//     // {   // s is not valid here as it is not declared
//     //     let s = "hello";    // s is valid from this points forward // here s is string literal
//     //     // do stuff with s
//     //     println!("{s}");
//     // }   // This scope is now over, and s is no longer valid

//     // ---The String--- 
//     // You can create a String from a string literal using the from function, like so:

//     // let mut s = String::from("hello");  // The double colon :: operator allows us to namespace this particular from function under the String type
//     // println!("Value of s is {s}");
//     // s.push_str(", world !");
//     // println!("Value of s is {s}");

//     // So, what’s the difference here? Why can String be mutated but literals cannot? The difference is in how these two types deal with memory.

//     // {
//     //     let s = String::from("hello"); // s is valid from this point forward

//     //     // do stuff with s
//     // }                                  // this scope is now over, and s is no longer valid
//     // There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

//     // ---Variables and Data Interacting with Move---
//     let x: i32=5;
//     let y:i32=x;
//     println!("The value of x is {x} and value of y is {y}"); // 5 5
//     // We can probably guess what this is doing: 
//     // “bind the value 5 to x; then make a copy of the value in x and bind it to y.” 
//     // We now have two variables, x and y, and both equal 5. 
//     // This is indeed what is happening, because integers are (simple values) with a known, fixed size, and these two 5 values are pushed onto the stack.

//     // String version
//     // A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
//     let s1 = String::from("hello");
//     let s2 = s1;    // Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1
//     println!("{s2}");
//     // s1 and s2 both points to the same heap
//     // print!("{s1}"); // throws error
//     // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
//     // If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.

// }

// ---Ownership and functions---
// fn main() {
//     let s = String::from("hello");
//     println!("{s}");
//     // passing s to fn 
//     takes_ownership(s);
//     // println!("{s}"); // throws errors "s" value moves into the function...

//     let x=5;
//     move_value(x);
//     println!("{x}");    //x would move into the function, but i32 is Copy so its ok to use x afterwards
// }
// fn takes_ownership(some_string:String) {
//     println!("{some_string}");
// }
// fn move_value(some_integer:i32) {
    //     println!("{some_integer}");
// }

// ---Return Values and Scope---
// Returning values can also transfer ownership. 
// fn main() {
//     let s1=gives_ownership();   // it moves its return value to s1
//     println!("s1 is {s1}");


    
// }
// fn gives_ownership() ->String {
//     let some_string = String::from("hello");
//     println!("Some string is {some_string}");
//     some_string
// }

// Rust does let us return multiple values using a tuple, as shown in Listing 4-5.


// fn main() {
//     let s1 = String::from("hello");
    
//     let (s2, len) = calculate_length(s1);
    
//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
    
//     (s, length)
// }
// Luckily for us, Rust has a feature for using a value without transferring ownership, called references.

// ====================================
// ---References and Borrowing---

// fn main () {
//     let s1=String::from("hello");
//     let len=calculate_length(&s1);
//     // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
//     // Because it does not own it, the value it points to will not be dropped when the reference stops being used.

//     println!("s1 is {s1} and len is {len}");

// }
// // the signature of the function uses & to indicate that the type of the parameter s is a reference.
// fn calculate_length(s:&String)->usize {
//     s.len()
// }   // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, it is not dropped.

// ---Mutable References---
// fn main() {
//     let mut s1=String::from("hello");
//     println!("s1 is {s1}"); // s1 is hello
//     change(&mut s1); 
//     println!("s1 is {s1}"); // s1 is hello, world

// }
// fn change(s:&mut String) {
//     s.push_str(", world");
// }
// This makes it very clear that the change function will mutate the value it borrows.
// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s;

// The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.

// let mut s = String::from("hello");
// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3);
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.
// ======================================

// ---Dangling references---
// In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does

// The Rules of References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.