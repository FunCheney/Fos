#[test]
fn test_01() {

    let s = "hello world".to_string();

    println!("addr of ss: {:p}, s: {:p}, len: {}, capacity: {}, size: {}",
             &"hello world", &s, s.len(), s.capacity(), std::mem::size_of_val(&s));
}

#[test]
fn test_02() {
    let mut  a = Vec::new();
    a.push(1);
    a.push(2);

    println!("addr of  a: {:p}, len: {}, capacity: {}, size: {}",
             &a, a.len(), a.capacity(), std::mem::size_of_val(&a));

}