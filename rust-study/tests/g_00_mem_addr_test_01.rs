use std::fmt::Debug;
use std::mem::{size_of_val, transmute};

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

#[test]
fn test_03() {
    let v = vec![1,2,3];
    let a: &Vec<u64> = &v;
    let b: &[u64] = &v;
    let c: &dyn Debug = &v;

    println!("a size: {}", size_of_val(&a));
    println!("b size: {}", size_of_val(&b));
    println!("c size: {}", size_of_val(&c));

    println!("a: {}", a as *const _ as usize);
    println!("b: {:?}", unsafe { transmute::<_, (usize, usize)>(b) });
    println!("c: {:?}", unsafe { transmute::<_, (usize, usize)>(c) });


}