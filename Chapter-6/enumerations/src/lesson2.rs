// ***** Rename the file to main.rs******
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

    // matches are exhaustive
    // Catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3=>fancy_hat(),
        7=>remove_hat(),
        other=>move_player(other),
    }
    fn fancy_hat() {}
    fn remove_hat() {}
    fn move_player(num_spaces:u8) {}
    // This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed. This catch-all pattern meets the requirement that match must be exhaustive


    // Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

    let roll_again = 9;
    match roll_again {
        3=>fancy_hat(),
        7=>remove_hat(),
        _=>other(),
    }
    fn other() {}

    // nothing else happens on your turn if you roll anything other than a 3 or a 7. We can express that by using the unit value (the empty tuple type 
    // _ => ()
    // Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this cas
}
