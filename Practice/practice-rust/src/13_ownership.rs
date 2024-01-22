// Define a function that takes ownership of a vector and prints its elements.

fn takes_ownership(arr:[i32; 5]) {
    for ele in arr {
        print!("{} ", ele);
    }
    
}
fn main() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    takes_ownership(arr);
    print!("{}", arr[0]);
}