// 17- Declare an array of integers and print each element using a for loop.

fn main() {
    let arr:[u32;5] = [1, 2, 3, 4, 5];
    
    for ele in arr {
        print!("{} ", ele);
    }
}