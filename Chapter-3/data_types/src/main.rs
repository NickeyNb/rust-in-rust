// 3.2
// Rust is a statically typed language, which means that it must know the types of all variables at compile time.
// Two data type subsets: scalar and compound.
// A scalar type represents a single value. 
// Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

// *** Scaler Types ***
// 1- Integer type
// 2- Floating type = The f32 type is a single-precision float, and f64 has double precision.
// 3- Boolean type = Booleans are one byte in size. The Boolean type in Rust is specified using bool.
// 4- Character type(4 byte) = Rust’s char type is the language’s most primitive alphabetic type. 
// Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.

// *** Compound Types ***
// Rust has two primitive compound types: tuples and arrays.
// 1- Tuple type = Tuples have a fixed length: once declared, they cannot grow or shrink in size
        // The tuple without any values has a special name, unit.
// 2- Array type = arrays in Rust have a fixed length.

fn main() {
    // ***Boolean***
    let male: bool = true;
    let female:bool = false;
    println!("{}", male);
    println!("{}", female);

    // *** Char***
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);

    // ***tuple***
    let tup:(i32, f32, u8)=(500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");

    let five_hundred = tup.0;   // tup . index
    let six_point_four = tup.1;

    // ***Array type***
    let nums = [1, 2, 3, 4, 5];
    // Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
    let a = [3;5];  // let a = [3, 3, 3, 3, 3];
    let first = a[0];
    
    
    
}
