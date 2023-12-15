// The match Control Flow Construct
// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main () {

}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny !");
            1
        }    
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
