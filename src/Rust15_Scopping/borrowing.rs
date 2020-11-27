// ownership moves
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// borrow
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // a boxed and a stacked
    let boxed_i32 = Box::new(5i32);
    let stacked_i32= 6i32;

    // ownership not taken
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // borrow ref
        borrow_i32(_ref_to_i32);
        // ref not borrow
    }

    eat_box_i32(boxed_i32);
}