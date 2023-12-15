// Defining and instantiating structures

// Defined the structure, instantiating it in main
// struct User {
//     // These are called fields
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

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
// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// ===============================

// Ownership of Struct Data

// ===============================

// 5.2
// An example program using structs
// area of rectangle

// fn main() {
//     let width1=30;
//     let height1=50;

//     println!("The area of rectangle is {}", area_of_rectangle(width1, height1));
// }

// fn area_of_rectangle(width1:i32, height1:i32)->i32 {
//     width1 * height1
// }
// ==============
// using tuple type
// fn main() {
//     let rect1 = (10, 20);
//     println!("The area is {}", area_rectangle(rect1));
// }

// fn area_rectangle(dimensions:(i32, i32))->i32 {
//     dimensions.0 * dimensions.1
// }

// ==================
// Refactoring with Structs: Adding More Meaning

// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)] just before the struct definition,


// #[derive(Debug)]
// struct Rectangle {
//     width:u32,
//     height:u32,
// }
// fn main () {
//     let rect1 = Rectangle{
//         width:10,
//         height:10,
//     };

//     // println!("The values is {:?}", rect1); //The values is Rectangle { width: 10, height: 10 }
//     println!("The area is {}", area(&rect1));
// }

// // Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. 
// fn area(rectangle:&Rectangle)->u32 {
//     rectangle.width * rectangle.height
// }
// // The area function accesses the width and height fields of the Rectangle instance (note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs).

// // ===========================================


// 5.3 Mehtod Syntax
// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapter 6 and Chapter 17, respectively), and 
// their first parameter is always self, which represents the instance of the struct the method is being called on.

// #[derive(Debug)]
// struct Rectangle {
//     width:u32,
//     height:u32,
// }

// impl Rectangle {
//     self which is the instance the method is called on here Rectangle reference
//     fn area(&self)->u32 {
//         self.width * self.height
//     }  
        
// }

// fn main() {
//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }
// To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type
// In the signature for area, we use &self instead of rectangle: &Rectangle. The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for. Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. 
//  Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API. 


