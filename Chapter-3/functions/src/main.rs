// 3.3
// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words
// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

fn main() {
    println!("Hello, world!");
    expression();
    another_function(5, 10);
}
// snake case 
fn another_function (x:i32, y:i32) {

    println!("I am another funciton with value {x} {y}");
    let val = five();
    println!("{val}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.
// Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

fn expression() {
    let y = {
        let x = 3;
        x + 1   // doesn't have semicolon
    };

    println!("The value of y is: {y}"); // 4
    
}
// Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 

// Functions with Return Values
// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly
fn five () ->i32 {
    5   // its an expression whose value we want to return
}