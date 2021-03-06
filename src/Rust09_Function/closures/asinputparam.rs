// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    F: FnOnce() {
    // ^ Fn? FnOnce? FnMut?
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        // greeting is ref
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);

        // Manually calling drop forces `farewell` to be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
