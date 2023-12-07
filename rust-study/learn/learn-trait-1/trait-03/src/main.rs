pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>; 
}
trait TraitA {
    type Mytype;
    
}

fn doit<T: TraitA>(_a: T::Mytype){}
struct TypeA;

impl TraitA for TypeA{
    type Mytype = String;
    
}


fn main() {
    doit::<TypeA>("".to_string());
    println!("Hello, world!");
}
