fn main() {
    for number in 1..=19 {
        println!("Tell me about {}", number);

        match number {
            1 => println!("One!"),
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            13..=19 => println!("A teen"),
            _ => println!("Ain't special"),
        }
        println!();
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    // guards
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    // binding using `@`

    fn age() -> u32 {
        15
    }
    match age() {
        0 => {}
        n @ 1..=12 => { println!("{}", n);}
        n @ 13..=19 => {println!("{}", n);}
        n => {}
    }

    // destructure via binding
    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n)      => println!("Not interesting... {}", n),
        _            => (),
    }
}