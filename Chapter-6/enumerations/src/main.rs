
// 6.1
// Defining an enum-enums give you a way of saying a value is one of a possible set of values.

// IpAddrKind is now a custom data type that we can use elsewhere in our code.
// enum IpAddrKind {
//     V4, 
//     V6
// }

// fn main() {
    // println!("Hello, world!");
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind :: V6;
    // route(IpAddrKind::V4)
// }


//
// This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback:IpAddr = IpAddr::V6(String::from("::1"));
}


