#[allow(dead_code)]
struct UnPrintable(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Printable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // There is no control over how the results look.
    println!("Now {:?} will print!", Deep(Structure(7)));

    // Use {:#?} to make the result more elegant
    println!("Now {:#?} will print!", Deep(Structure(7)));
}