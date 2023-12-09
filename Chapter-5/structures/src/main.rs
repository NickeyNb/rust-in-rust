// Defining and instantiating structures

// Defined the structure, instantiating it in main
struct User {
    // These are called fields
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() {
//     // instantiating User struct
//     // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. 
//     // let mut user = User{
//     //     active:false,
//     //     username:String::from("First"),
//     //     email:String::from("first@first.com"),
//     //     sign_in_count:0,
//     // };
//     // println!("{}", user.email);
//     // // since we have marked the instance as mut we can change it
//     // user.email=String::from("second@second.com");
//     // println!("{}", user.email);
    
//     // calling instance via function call
//     let email2=String::from("user2@user2.com");
//     let username2 = String::from("username2");
//     let user2 = build_user(email2, username2);
//     println!("{}, {}, {}, {}", user2.active, user2.email, user2.username, user2.sign_in_count);
// }

// fn build_user(email:String, username:String)->User {
//     User{
//         active:true,
//         username:username,
//         email,  // used the shordhand, field init(same name of key and value)
//         sign_in_count:1,
//     }
// }

// ============================================
// Creating instances from Other instances with struct update syntax
// fn main() {
//     let user1 = User{
//         active:true,
//         username:String::from("user1"),
//         email:String::from("user1@user.com"),
//         sign_in_count:10,
//     };
//     // 
//     let user2 = User{
//         email:String::from("user2@user.com"),
//         ..user1
//         // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
//         // ..user1 must come last
//     };

//     println!("{}, {}", user2.email, user2.username);

// }
// =================================
// Using Tuple Structs Without Named Fields to Create Different Types   

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black: Color = Color(0, 0, 0);
//     let origin: Point = Point(0, 0, 0);
// }
// Each struct you define is its own type, even though the fields within the struct might have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
// ======================================================

// You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

// ===============================

// Ownership of Struct Data
