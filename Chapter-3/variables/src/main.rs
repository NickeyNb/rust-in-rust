

// 3.1-variables and mutability
fn main() {
    // ***Variables*** are immutable by default 
    let mut x=5;
    println!("The value of x is {x}");
    x=6;
    println!("The value of x is {x}");

    // ***Constants*** are immutable in nature
    const THREE_HOUR_IN_SEC:u32 = 3 * 60 * 60;
    println!("{}, {}", THREE_HOUR_IN_SEC, x);


    // ***Shadowning***  the second variable overshadows the first.
    // it is different from mut as in mut we can't change the type
    // let mut spaces = "    ";
    // spaces = spaces.len();  // error

    let spaces = "    ";    // string
    let spaces = spaces.len(); // number
    println!("The value of spaces is {spaces}");
}