trait TraitA {
    const LEN: u32 = 10;
}
struct A;

impl TraitA for A {
    const LEN: u32 = 28;
    
}
fn main() {
    println!("{:?}",A::LEN);
    println!("{:?}",<A as TraitA>::LEN);
    println!("Hello, world!");
}
