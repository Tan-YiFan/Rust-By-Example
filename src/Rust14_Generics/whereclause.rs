// specifying generic types and bounds separately is clearer
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause
/*
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
*/
// Another usage

use std::fmt::Debug;

trait Print{
    fn print(self);
}

impl <T> Print for T where Option<T>:Debug {
    fn print(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print();
}
