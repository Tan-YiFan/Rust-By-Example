// convert a type to string

use std::fmt::{Display, Formatter};
use std::fmt;

struct Circle {
    radius: i32
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Circle of the radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // parsing a string
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

