fn main() {

    let s1 = gives_ownership();


    println!("s1 = {}", s1);

    let s2 = String::from("helllo");

    let s3 = takes_and_gives_back(s2);


    //println!("s2 = {}", s2);
    println!("s2 = {}", s2);
    println!("Hello, world!");
}

fn gives_ownership() -> String{
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String{
    s
}

