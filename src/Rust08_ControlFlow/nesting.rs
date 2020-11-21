// `break` outer or inner loop

#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    let mut counter = 0;
    let k = loop {
        counter += 1;
        if counter == 10 {
            break counter << 1;
        }
    };

    assert_eq!(k, 20);
}