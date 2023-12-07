// 
fn main() {
    let ref_s = dangle();
    println!("Hello, world!");
}

fn dangle() -> &String{
    let s = String::from("hello");
    &s
}
