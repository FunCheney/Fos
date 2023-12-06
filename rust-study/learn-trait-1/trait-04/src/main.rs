use std::fmt::Debug;

trait TriatA{
    type Item: Debug;
}

#[derive(Debug)]
struct A;

struct B;

impl TriatA for B {
    type Item = A;
    
}

fn doit<T>()
    where
        T: TriatA,
        T::Item: Debug + PartialEq
{}

fn main() {
    println!("Hello, world!");
}
