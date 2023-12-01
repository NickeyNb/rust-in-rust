fn main() {
    // -----Ownership rules-----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped

    // {   // s is not valid here as it is not declared
    //     let s = "hello";    // s is valid from this points forward // here s is string literal
    //     // do stuff with s
    //     println!("{s}");
    // }   // This scope is now over, and s is no longer valid

    // ---The String--- 
    // You can create a String from a string literal using the from function, like so:

    // let mut s = String::from("hello");  // The double colon :: operator allows us to namespace this particular from function under the String type
    // println!("Value of s is {s}");
    // s.push_str(", world !");
    // println!("Value of s is {s}");

    // So, what’s the difference here? Why can String be mutated but literals cannot? The difference is in how these two types deal with memory.

    // {
    //     let s = String::from("hello"); // s is valid from this point forward

    //     // do stuff with s
    // }                                  // this scope is now over, and s is no longer valid
    // There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    // ---Variables and Data Interacting with Move---
    let x: i32=5;
    let y:i32=x;
    println!("The value of x is {x} and value of y is {y}"); // 5 5
    // We can probably guess what this is doing: 
    // “bind the value 5 to x; then make a copy of the value in x and bind it to y.” 
    // We now have two variables, x and y, and both equal 5. 
    // This is indeed what is happening, because integers are (simple values) with a known, fixed size, and these two 5 values are pushed onto the stack.

    // String version
    // A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
    let s1 = String::from("hello");
    let s2 = s1;    // Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1
    // s1 and s2 both points to the same heap
    // print!("{s1}"); // throws error
    // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
    // If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
    




    

}
