// 18- Find the sum of all elements in an array.

fn main() {
    let arr:[u32;5] = [1, 2, 3, 4, 5];
    let mut i=0;
    let mut sum=0;

    while i<arr.len() {
        sum += arr[i];
        i+=1;
    }

    println!("Sum is {}", sum);
}