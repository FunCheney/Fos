fn main() {
    let a = 1;
    println!("a = {}", a);

    let mut b: u32 = 1;
    println!("b = {}", b);
    
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a_binding = {}", a_binding);
    println!("Hello, world!");

}
