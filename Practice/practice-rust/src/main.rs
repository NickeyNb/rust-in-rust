// 8- Implement a simple rock-paper-scissors game where the user can input their choice.
// change to main.rs

use  std::io;
fn main() {
    println!("Let's start, rock-paper-scissor");
    let mut one = String::new();
    let mut two = String::new();

    println!("User one turn ! ");
    io::stdin().read_line(&mut one).expect("Error in turn one ");
    one = one.trim().to_string();

    println!("User two turn ! ");
    io::stdin().read_line(&mut two).expect("Error in turn two ");
    two = two.trim().to_string();

    rockpaperscissor(one, two);

}

fn rockpaperscissor(one:String, two:String) {
    if one == two {
        println!("Draw game ! ");
    } else if (one == "rock" && two=="scissor") || (one == "paper" && two=="rock") || (one == "scissor" && two=="paper") {
        println!("One wins ! ");
    } else {
        println!("Two wins ! ");
    }
}