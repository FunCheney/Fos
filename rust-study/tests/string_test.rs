#[test]
fn string_01() {
    let s = String::from("hello, world");
    println!("{}", s);
    let aa = s.to_string();
    println!("{}", aa);
    let a = &"hello"[0..1];

    let b = &"";

    let c = "";
}