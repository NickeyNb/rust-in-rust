// 24- Use a loop to print the multiplication table for a given number.

fn main() {
    let num:u32 = 2;
    let mut i:u32 = 1;

    println!("Multiplication table of {}", num);
    while i<=10 {
        println!("{} * {} = {}", num, i, num*i);
        i+=1;
    }
}