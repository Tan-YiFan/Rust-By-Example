use std::fmt;

struct Structure(i32);


mod display {

}
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}