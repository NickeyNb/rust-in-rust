// 20- Write a program that searches for a specific element in an array and prints its index.

fn main() {
    let arr:[i32;5] = [1, -2, 3, -4, 5];
    let key: i32 = 11;

    let mut i=0;
    while i<arr.len() {
        if arr[i] == key {
            println!("Index is : {}", i);
            return;
        }
        i = i+1;
    }
    println!("Element not found ! ");
}