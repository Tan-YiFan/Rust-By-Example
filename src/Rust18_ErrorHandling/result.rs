// Ok(T)
// Err(E)

use std::num::ParseIntError;

#[allow(dead_code)]
fn error_function(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn fix_error(s: &str) -> Result<i32, ParseIntError> {
    // match s.parse::<i32>() {
    //     Ok(t) => Ok(t),
    //     Err(e) => Err(e)
    // }

    // using ?
    Ok(s.parse::<i32>()?)
}

fn main() {
    // let two = error_function("tg");
    let two = fix_error("r");
    match two {
        Ok(t) => println!("{}", t),
        Err(e) => println!("Err: {}", e)
    }
}