fn main() {
    let mut _mutable_integer = 7i32;
    {
        // shadowing the immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // error
        // _mutable_integer = 50;
    }

    // OK
    _mutable_integer = 3;
}