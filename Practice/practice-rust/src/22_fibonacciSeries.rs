// 22- Write a program that prints the first 10 numbers in the Fibonacci sequence.

fn fibonacci_series(mut first:u32,mut second:u32, n:u32) {

    let mut i:u32=2;

    print!("{} {} ", first, second);

    while i < n {
        let third:u32 = first + second;
        first = second;
        second=third; 

        print!("{} ", third);

        i=i+1;
    } 
}

fn main() {
    let f:u32=0;
    let s:u32=1;
    let n = 10;
    
    fibonacci_series(f, s, n)
}