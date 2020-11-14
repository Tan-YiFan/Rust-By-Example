// TODO: Read https://doc.rust-lang.org/std/fmt/

fn main() {
    println!("{} days", 31); // 31 is of type i32. 31i64 is the same as 31ll

    // Positional Arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named Arguments
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "The quick brown fox",
             verb = "jumps over");

    // Special formatting after a ':'
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // Right aligned text with a specified width. Alignment can be <, ^, >
    println!("{number:>width$}", number = 1, width = 6);

    // Pad numbers with extra zeros
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust checks to make sure the correct numbers of arguments are used.
    // println!("My name is {0}, {1} {0}", "Jimmy"/*, "Tan"*/);

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This structure {} won't print!", Structure(3));

    // Activity: print pi
    let pi = std::f64::consts::PI;
    println!("Pi is roughly {pi:.width$}", pi = pi, width = 3);
}