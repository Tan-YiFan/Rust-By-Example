// uninitialized variables are not allowed
fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // initialize the binding
        a_binding = x * x;
    }
    println!("a_binding: {}", a_binding);

    let another_binding;
    // error
    // println!("another_binding: {}", another_binding);

    another_binding = 1;
    println!("another_binding: {}", another_binding);
}