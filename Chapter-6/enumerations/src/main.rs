// Concise control flow with if let
fn main () {
    let config_max = Some(3u8);
    match config_max {
        Some(max)=>println!("The max value configured is {}", max),
        _=>(),
    }

    // Instead we can use the shorter way using if let
    let config_max_2 = Some(3u8);
    if let Some(max)=config_max_2 {
        println!("The max value configured is {}", max);
    }
}