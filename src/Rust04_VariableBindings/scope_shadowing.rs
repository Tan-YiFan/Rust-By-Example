fn main() {
    let long_lived_binding = 1;

    // smaller scope
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // wrong
    // println!("outer short:{}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    // shadow
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}