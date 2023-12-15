
// 6.1
// Defining an enum-enums give you a way of saying a value is one of a possible set of values.

// IpAddrKind is now a custom data type that we can use elsewhere in our code.
enum IpAddrKind {
    // This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
    V4(String), 
    V6(u8, u8, u8, u8, u8, u8)
}
// This enum has four variants with different types:
// Quit has no data associated with it at all.
// Move has named fields, like a struct does(anonymous struct).
// Write includes a single String.
// ChangeColor includes three i32 values.

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}
enum Option<T> {
    Some(T),    // store some generic value
    None,   // no value
}
fn main() {
    // println!("Hello, world!");
    // let four = IpAddrKind::V4;  // variants are namespaced under their identifier hence ::,  
    // let six = IpAddrKind :: V6;

    let localhost1: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
    let localhost2:IpAddrKind = IpAddrKind::V6(1, 1, 1, 1, 1, 1);

    let some_number = Some(5);
    let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // Throws error
}


//
