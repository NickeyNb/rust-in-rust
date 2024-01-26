// 23- Use a loop to iterate over a range of 0 to 20 and print whether each number is even or odd.

fn main() {
    let mut st:u32 =0;
    let end:u32 = 20;

    while st<=end {
        if st%2 == 0 {
            println!("{} is even!", st);
        } else {
            println!("{} is odd ! ", st);
        }

        st+=1;
    }
}