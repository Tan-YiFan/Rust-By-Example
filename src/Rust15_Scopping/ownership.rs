fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed
}

fn main() {
    // on stack
    let x = 5u32;

    // no resource is moved
    let y = x;

    // on heap
    let a = Box::new(5i32);
    println!("a contains {}", a);

    // move
    // can add mutability to b
    let b = a;

    // now a is moved
    // println!("a contains {}", a);

    // b is moved
    destroy_box(b);
    // memory is freed

    // println!("b contains {}", b);

}