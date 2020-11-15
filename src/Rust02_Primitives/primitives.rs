fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let default_float = 3.0;
    let default_integer = 7;
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // mutable=true; error
    let mutable = true; // overwritten is ok
}