fn main() {
    let s1 = String::from("hello");
    let len = calcute_length(&s1);

    println!("len = {}", len);
    println!("Hello, world!");
}

fn calcute_length(s: &String) ->usize{
    s.len()
}

