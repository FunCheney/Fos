fn takes_ownership(some_thing: String) -> String{
    println!("{}", some_thing);
    some_thing
}

fn makes_copy(i: i32){

    println!("i = {}", i);
}

fn main() {
   
    let s = String::from("hello");
    let s1 = takes_ownership(s);

    println!("s1 = {}", s1);

    let x = 5;
    makes_copy(x);
    
    println!("{}", x);
    println!("Hello, world!");
}
