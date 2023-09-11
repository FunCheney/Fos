fn main() {

    let x : i32 = 1;
    {
        let y : i32 = 1;
        println!("y = {}", y);
        println!("x = {}", x);
    }
    {
        let mut s1 = String::from("hello");
        s1.push_str(" world");
        println!("s1 = {}", s1);

        let s2 = s1;
        println!("s2 = {}", s2);

        let s3 = s2.clone();
        println!("s3 = {}", s3);
        println!("s2 = {}", s2);
    }

    // copy trait  
    let a = 1;
    let b = a;

    println!("a = {}, b = {}", a, b);

    println!("Hello, world!");
}
