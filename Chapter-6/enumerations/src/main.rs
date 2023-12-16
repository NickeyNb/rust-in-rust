// The match Control Flow Construct
// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 
// Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. 

#[derive(Debug)]
enum  UsStates {
    Alaska,
    Alabama,
    
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny !");
            1
        }    
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x:Option<i32>)->Option<i32> {
    match x {
        None=>None,
        Some(i)=>Some(i+1),
        // The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
    }
}
fn main () {
    println!("{}" ,value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsStates::Alabama)));

    let five = Some(5);
    let six = plus_one(five);   // plus_one(Some(5));
    let none = plus_one(None);
}
