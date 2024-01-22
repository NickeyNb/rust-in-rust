// 15- Create a function that takes a string slice and appends another string to it.

fn append_str(var:&str) {
    let adding = "added string slice";
    let new_str = format!("{var} {adding}");
    println!("{}", new_str);
}
fn main() {
    let  var = "Nickey";
    
    append_str(var);  
}