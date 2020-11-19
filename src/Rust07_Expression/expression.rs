// the most common two expressions are:
//  declaring a varibable binding
//  using an `;` with an expression

fn test() {
    let x = 5;

    x;
    x + 1;
    15;
}

// blocks are expressions too.
// The last expression is the return value
// It the last expression ends with `;`, the return value would be ()

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}