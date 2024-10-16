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

    // 8
    println!("a size: {}", size_of_val(&a));
    // 16
    println!("b size: {}", size_of_val(&b));
    // 16
    println!("c size: {}", size_of_val(&c));

    // 105553175429120
    println!("v: {}", v.as_ptr() as *const _ as usize);

    // 123145404744848  --> pointer
    println!("a: {}", a as *const _ as usize);
    // (105553175429120, 3) --> (pointer, length)
    // slice 包含支撑指针和元素的个数
    println!("b: {:?}", unsafe { transmute::<_, (usize, usize)>(b) });
    // (123145404744848, 4352086488)
    // 特征对象的元数据是 DynMetadata<Self>，其中 Self 是特征对象的类型。这个元数据包含了特征对象的类型信息，
    // 以及可能的其他元数据，如 vtable（虚表）指针。
    println!("c: {:?}", unsafe { transmute::<_, (usize, usize)>(c) });
}